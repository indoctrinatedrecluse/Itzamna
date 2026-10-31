package ixchel.domain

import zio.json.*

final case class DoraMetrics(
    deploymentFrequency: Double, // deploys per day
    leadTimeHours: Double,       // hours from commit to production
    changeFailureRate: Double,   // percentage (0.0 to 100.0)
    mttrMinutes: Double,         // mean time to recovery in minutes
    tier: String                 // "Elite", "High", "Medium", "Low"
)

object DoraMetrics:
  given JsonCodec[DoraMetrics] = DeriveJsonCodec.gen[DoraMetrics]

final case class VelocityMetrics(
    rolling30dCommits: Int,
    rolling90dCommits: Int,
    commitsPerDay: Double,
    prCycleTimeHours: Double,
    reviewDepthRatio: Double
)

object VelocityMetrics:
  given JsonCodec[VelocityMetrics] = DeriveJsonCodec.gen[VelocityMetrics]

final case class ChurnMetrics(
    linesAdded: Long,
    linesDeleted: Long,
    linesModified: Long,
    netGrowth: Long
)

object ChurnMetrics:
  given JsonCodec[ChurnMetrics] = DeriveJsonCodec.gen[ChurnMetrics]

final case class ContributorInsight(
    name: String,
    email: String,
    commitCount: Int,
    ownershipPercent: Double,
    activeDays: Int
)

object ContributorInsight:
  given JsonCodec[ContributorInsight] = DeriveJsonCodec.gen[ContributorInsight]

final case class BusFactor(
    score: Int,
    keyHolders: List[String],
    riskLevel: String // "Low", "Moderate", "Critical"
)

object BusFactor:
  given JsonCodec[BusFactor] = DeriveJsonCodec.gen[BusFactor]

final case class RepoHealthOverview(
    repo: RepoMetadata,
    healthScore: Int, // 0 - 100
    dora: DoraMetrics,
    velocity: VelocityMetrics,
    churn: ChurnMetrics,
    topContributors: List[ContributorInsight],
    busFactor: BusFactor,
    lastSyncTimestamp: Long
)

object RepoHealthOverview:
  given JsonCodec[RepoHealthOverview] = DeriveJsonCodec.gen[RepoHealthOverview]

// --- WebSocket Telemetry Events ---

sealed trait TelemetryEvent

object TelemetryEvent:
  final case class Pulse(
      timestamp: Long,
      activeFibers: Int,
      healthScore: Int,
      connectedClients: Int
  ) extends TelemetryEvent

  final case class CommitMined(
      repo: String,
      sha: String,
      author: String,
      message: String
  ) extends TelemetryEvent

  final case class MetricsUpdated(
      overview: RepoHealthOverview
  ) extends TelemetryEvent

  final case class SacredAlert(
      level: String, // "INFO", "WARNING", "CELESTIAL"
      message: String,
      timestamp: Long
  ) extends TelemetryEvent

  given JsonCodec[Pulse] = DeriveJsonCodec.gen[Pulse]
  given JsonCodec[CommitMined] = DeriveJsonCodec.gen[CommitMined]
  given JsonCodec[MetricsUpdated] = DeriveJsonCodec.gen[MetricsUpdated]
  given JsonCodec[SacredAlert] = DeriveJsonCodec.gen[SacredAlert]
  given JsonCodec[TelemetryEvent] = DeriveJsonCodec.gen[TelemetryEvent]
