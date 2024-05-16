ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.4.2"

lazy val root = (project in file("."))
  .settings(
    name := "scala-functional-programming-my-playground",
    libraryDependencies ++= Seq("org.scalatest" %% "scalatest" % "3.2.18" % Test)
  )
