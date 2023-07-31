package playground.booookstore

import io.ktor.server.application.*
import playground.booookstore.plugins.configureRouting
import playground.booookstore.plugins.configureSerialization

fun main(args: Array<String>): Unit = io.ktor.server.netty.EngineMain.main(args)

fun Application.module() {
    configureRouting()
    configureSerialization()
}
