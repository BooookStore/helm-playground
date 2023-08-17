package playground.booookstore.query.usecase

import kotlinx.coroutines.async
import kotlinx.coroutines.coroutineScope
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.Serializable
import playground.booookstore.query.gateway.ItemQueryGateway
import playground.booookstore.query.gateway.OrderQueryGateway
import playground.booookstore.query.gateway.ShopQueryGateway
import playground.booookstore.query.gateway.ShopQueryModel
import playground.booookstore.query.type.ItemId
import playground.booookstore.query.type.OrderId
import playground.booookstore.query.type.ShopId

class OrderQueryUsecase(
    private val orderQueryGateway: OrderQueryGateway,
    private val shopQueryGateway: ShopQueryGateway,
    private val itemQueryGateway: ItemQueryGateway,
) {

    suspend fun execute(orderId: OrderId) = coroutineScope {
        val orderDeferred = async { orderQueryGateway.find(orderId) }
        val shopDeferred = async { shopQueryGateway.find(orderId) }
        val itemsDeferred = async { itemQueryGateway.find(orderId) }

        val order = orderDeferred.await()
        val shop = shopDeferred.await()
        val items = itemsDeferred.await()

        OrderQueryView(
            id = order.id,
            orderDateTime = order.orderDateTime,
            shop = ShopQueryView(id = shop.id, name = shop.name),
            items = items.set.map { OrderQueryViewItem(id = it.id, name = it.name) }.toMutableList()
        )
    }

}

@Serializable
data class OrderQueryView(
    val id: OrderId,
    val orderDateTime: LocalDateTime,
    val shop: ShopQueryView,
    val items: MutableList<OrderQueryViewItem>,
)

@Serializable
data class ShopQueryView(
    val id: ShopId,
    val name: String,
)

@Serializable
data class OrderQueryViewItem(
    val id: ItemId,
    val name: String
)