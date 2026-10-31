package ixchel.api

import ixchel.domain.*
import ixchel.service.{AnalyticsEngine, EventStream}
import zio.*
import zio.http.*
import zio.json.*

object RestRoutes:

  final case class HealthStatus(
      status: String,
      service: String,
      version: String,
      activeClients: Int,
      timestamp: Long
  )
  object HealthStatus:
    given JsonCodec[HealthStatus] = DeriveJsonCodec.gen[HealthStatus]

  val routes: Routes[AnalyticsEngine & EventStream, Nothing] =
    Routes(
      // --- Health & Sanctuary Heartbeat ---
      Method.GET / "health" -> handler {
        for
          stream  <- ZIO.service[EventStream]
          clients <- stream.activeClients
          now     <- Clock.currentTime(java.util.concurrent.TimeUnit.MILLISECONDS)
          payload  = HealthStatus(
            status = "CELESTIAL_ALIVE",
            service = "Ixchel Sacred Analytics Conduit",
            version = "0.1.0",
            activeClients = clients,
            timestamp = now
          )
        yield Response.json(payload.toJson)
      },

      // --- DORA Metrics ---
      Method.GET / "api" / "v1" / "analytics" / "dora" -> handler {
        (for
          engine <- ZIO.service[AnalyticsEngine]
          dora   <- engine.computeDora(RepoId("Itzamna"))
        yield Response.json(dora.toJson)).catchAll { err =>
          ZIO.succeed(Response.internalServerError(s"Celestial anomaly: ${err.getMessage}"))
        }
      },

      // --- Velocity & Churn ---
      Method.GET / "api" / "v1" / "analytics" / "velocity" -> handler {
        (for
          engine <- ZIO.service[AnalyticsEngine]
          vel    <- engine.computeVelocity(RepoId("Itzamna"))
        yield Response.json(vel.toJson)).catchAll { err =>
          ZIO.succeed(Response.internalServerError(s"Celestial anomaly: ${err.getMessage}"))
        }
      },

      // --- Contributors & Bus Factor ---
      Method.GET / "api" / "v1" / "analytics" / "contributors" -> handler {
        (for
          engine   <- ZIO.service[AnalyticsEngine]
          contribs <- engine.computeContributors(RepoId("Itzamna"))
        yield Response.json(contribs.toJson)).catchAll { err =>
          ZIO.succeed(Response.internalServerError(s"Celestial anomaly: ${err.getMessage}"))
        }
      },

      // --- Master Repository Overview ---
      Method.GET / "api" / "v1" / "analytics" / "overview" -> handler {
        (for
          engine <- ZIO.service[AnalyticsEngine]
          ov     <- engine.computeOverview(RepoId("Itzamna"))
        yield Response.json(ov.toJson)).catchAll { err =>
          ZIO.succeed(Response.internalServerError(s"Celestial anomaly: ${err.getMessage}"))
        }
      },

      // --- Trigger Manual Sync Ritual ---
      Method.POST / "api" / "v1" / "sync" -> handler {
        (for
          engine <- ZIO.service[AnalyticsEngine]
          stream <- ZIO.service[EventStream]
          ov     <- engine.computeOverview(RepoId("Itzamna"))
          _      <- stream.broadcast(TelemetryEvent.MetricsUpdated(ov))
          _      <- stream.broadcast(
                      TelemetryEvent.SacredAlert(
                        level = "CELESTIAL",
                        message = "Ixchel synchronized the cosmic threads for Itzamna repository.",
                        timestamp = ov.lastSyncTimestamp
                      )
                    )
        yield Response.json(ov.toJson)).catchAll { err =>
          ZIO.succeed(Response.internalServerError(s"Celestial anomaly: ${err.getMessage}"))
        }
      }
    )
