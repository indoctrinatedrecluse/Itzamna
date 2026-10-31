package ixchel

import ixchel.api.{RestRoutes, WebSocketRoutes}
import ixchel.domain.*
import ixchel.provider.TelemetrySynthesizer
import ixchel.service.{AnalyticsEngine, AnalyticsEngineLive, EventStream, EventStreamLive}
import zio.*
import zio.http.*

object Main extends ZIOAppDefault:

  private val banner: String =
    """
      |  ==============================================================
      |   🌙 IXCHEL : THE CELESTIAL WEAVER OF GIT PROPHECIES 🌙
      |   Scala 3.3 · ZIO 2 · ZIO HTTP 3 · ZIO Streams · DORA Telemetry
      |  ==============================================================
      |   Mayan Goddess of the Moon, Medicine, Water & Galactic Weaving
      |   Listening on : http://127.0.0.1:9090
      |   Endpoints    : GET /health
      |                : GET /api/v1/analytics/overview
      |                : GET /api/v1/analytics/dora
      |                : GET /api/v1/analytics/velocity
      |                : GET /api/v1/analytics/contributors
      |                : WS  /ws/telemetry
      |  ==============================================================
      |""".stripMargin

  // Background fiber emitting pulse heartbeats and telemetry updates
  private def startCelestialTicker: ZIO[AnalyticsEngine & EventStream, Nothing, Fiber.Runtime[Nothing, Nothing]] =
    val ticker =
      for
        _       <- ZIO.sleep(5.seconds)
        stream  <- ZIO.service[EventStream]
        engine  <- ZIO.service[AnalyticsEngine]
        clients <- stream.activeClients
        now     <- Clock.currentTime(java.util.concurrent.TimeUnit.MILLISECONDS)
        pulse    = TelemetryEvent.Pulse(
          timestamp = now,
          activeFibers = 4,
          healthScore = 96,
          connectedClients = clients
        )
        _       <- stream.broadcast(pulse)
        // Periodic metric broadcast every 30 seconds
        broadcastUpdate = if (now / 1000) % 30 < 5 then
          engine.computeOverview(RepoId("Itzamna")).flatMap { ov =>
            stream.broadcast(TelemetryEvent.MetricsUpdated(ov))
          }.catchAll(_ => ZIO.unit)
        else
          ZIO.unit
        _ <- broadcastUpdate
      yield ()

    ticker.forever.forkDaemon

  override val run: ZIO[Environment & ZIOAppArgs & Scope, Any, Any] =
    val allRoutes = RestRoutes.routes ++ WebSocketRoutes.routes

    val serverConfig = Server.Config.default.port(9090)

    val program =
      for
        _       <- Console.printLine(banner)
        _       <- startCelestialTicker
        _       <- Console.printLine("✨ Ixchel conduit awakened. Radiant moonbeams streaming on port 9090.")
        _       <- Server.serve(allRoutes)
      yield ()

    program.provide(
      ZLayer.succeed(serverConfig),
      Server.live,
      EventStreamLive.layer,
      TelemetrySynthesizer.live,
      AnalyticsEngineLive.layer
    )
