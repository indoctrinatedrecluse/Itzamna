package ixchel.api

import ixchel.domain.*
import ixchel.service.{AnalyticsEngine, EventStream}
import zio.*
import zio.http.*
import zio.http.WebSocketFrame
import zio.json.*

object WebSocketRoutes:

  val routes: Routes[AnalyticsEngine & EventStream, Nothing] =
    Routes(
      Method.GET / "ws" / "telemetry" -> handler {
        Handler.webSocket { channel =>
          for
            engine <- ZIO.service[AnalyticsEngine]
            stream <- ZIO.service[EventStream]
            _      <- stream.incrementClients
            // 1. Send initial telemetry state immediately upon connection
            overview <- engine.computeOverview(RepoId("Itzamna")).catchAll(_ =>
              ZIO.dieMessage("Failed initial overview")
            )
            initEvent: TelemetryEvent = TelemetryEvent.MetricsUpdated(overview)
            _        <- channel.send(ChannelEvent.Read(WebSocketFrame.text(initEvent.toJson)))

            // 2. Start sender fiber subscribed to the event stream
            senderFiber <- ZIO.scoped {
              stream.subscribe.flatMap { queue =>
                queue.take.flatMap { event =>
                  channel.send(ChannelEvent.Read(WebSocketFrame.text(event.toJson))).catchAll(_ => ZIO.unit)
                }.forever
              }
            }.forkDaemon

            // 3. Receive client messages (pings / requests)
            _ <- channel.receive.flatMap {
              case ChannelEvent.Read(WebSocketFrame.Text(text)) =>
                if text.contains("SYNC") then
                  for
                    updated <- engine.computeOverview(RepoId("Itzamna")).catchAll(_ => ZIO.dieMessage("Sync failed"))
                    _       <- stream.broadcast(TelemetryEvent.MetricsUpdated(updated))
                  yield ()
                else
                  ZIO.unit
              case ChannelEvent.Read(WebSocketFrame.Ping) =>
                channel.send(ChannelEvent.Read(WebSocketFrame.Pong))
              case ChannelEvent.Unregistered =>
                ZIO.unit
              case _ =>
                ZIO.unit
            }.forever.ensuring(
              senderFiber.interrupt *> stream.decrementClients
            )
          yield ()
        }.toResponse
      }
    )
