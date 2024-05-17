import Size.*

import scala.util.Try

case class Drink(name: Name, size: Size)

enum Size {
  case Large
  case Medium
  case Small
}

opaque type Name = String

object Name {

  def apply(s: String): Name = s

  extension (name: Name) def rawString: String = name

}

def extractDrinks(rawDrinks: String): Either[String, List[Drink]] = {
  val initializer: Either[String, List[Drink]] = Right(List.empty)
  rawDrinks
    .split(',')
    .map(_.trim)
    .map(extractDrink)
    .foldLeft(initializer)((acc, mayDrink) =>
      for {
        drinks <- acc
        drink <- mayDrink
      } yield drinks.appended(drink)
    )
}

def extractDrink(rawDrink: String): Either[String, Drink] = {
  val isDrinkOrderType = (d: String) => isOrderType(rawDrink, "D").filterOrElse(identity, "not drink order type")
  for {
    _ <- isDrinkOrderType(rawDrink)
    orderTypeRemovedRawDrink = rawDrink.replace("[D]", "").trim
    name <- extractDrinkNameWithSize(orderTypeRemovedRawDrink).orElse(extractDrinkNameWithoutSize(orderTypeRemovedRawDrink))
    size <- extractDrinkSize(orderTypeRemovedRawDrink).orElse(extractDrinkSizeWithoutSize(orderTypeRemovedRawDrink))
  } yield Drink(name, size)
}

private def isOrderType(rawOrder: String, expectedOrderType: String): Either[String, Boolean] = {
  val squareBracketOpen = rawOrder.indexOf('[')
  val squareBracketClose = rawOrder.indexOf(']')
  for {
    rawOrderType <- if (squareBracketOpen != -1 && squareBracketClose > squareBracketOpen + 1)
      Right(rawOrder.substring(squareBracketOpen + 1, squareBracketClose).trim)
    else
      Left(s"can't extract order type from $rawOrder")
    isMatchExpectedOrderType = rawOrderType.equals(expectedOrderType)
  } yield isMatchExpectedOrderType
}

private def extractDrinkSize(rawDrink: String): Either[String, Size] = {
  val bracketOpen = rawDrink.indexOf('(')
  val bracketClose = rawDrink.indexOf(')')
  for {
    rawSize <- if (bracketOpen != -1 && bracketClose > bracketOpen + 1)
      Right(rawDrink.substring(bracketOpen + 1, bracketClose))
    else
      Left(s"can't extract drink size from $rawDrink")
    size <- Try(Size.valueOf(rawSize)).toEither.left.map(_ => s"can't extract drink size from $rawDrink")
  } yield size
}

private def extractDrinkSizeWithoutSize(rawDrink: String): Either[String, Size] = {
  val bracketOpen = rawDrink.indexOf('(')
  val bracketClose = rawDrink.indexOf(')')
  if (bracketOpen == -1 && bracketClose == -1)
    Right(Medium)
  else
    Left(s"drink size specified or incorrect is $rawDrink")
}

private def extractDrinkNameWithSize(rawDrink: String): Either[String, Name] = {
  val bracketOpen = rawDrink.indexOf('(')
  if (bracketOpen != -1 && bracketOpen > 0)
    Right(Name(rawDrink.substring(0, bracketOpen).trim))
  else
    Left(s"can't extract drink name from $rawDrink")
}

private def extractDrinkNameWithoutSize(rawDrink: String): Either[String, Name] = {
  if (rawDrink.trim.nonEmpty)
    Right(Name(rawDrink.trim))
  else
    Left(s"can't extract drink name from $rawDrink")
}
