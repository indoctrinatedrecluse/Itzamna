package ixchel.provider

import ixchel.domain.*
import zio.*

final class TelemetrySynthesizer extends GitProvider:
  override val providerKind: ProviderKind = ProviderKind.GitHub

  override def fetchRepoMetadata(repoId: RepoId): Task[RepoMetadata] =
    ZIO.succeed(
      RepoMetadata(
        id = repoId,
        name = "Itzamna",
        owner = "indoctrinatedrecluse",
        defaultBranch = "main",
        provider = ProviderKind.GitHub,
        isPrivate = false,
        url = "https://github.com/indoctrinatedrecluse/Itzamna"
      )
    )

  override def fetchCommits(repoId: RepoId, limit: Int = 100): Task[List[GitCommit]] =
    Clock.currentTime(java.util.concurrent.TimeUnit.MILLISECONDS).map { now =>
      val authors = List(
        GitAuthor("Ahau Architect", "recluse@itzamna.io"),
        GitAuthor("Ixchel Priestess", "ixchel@celestial.dev"),
        GitAuthor("Kukulkan Weave", "feathered@serpent.myth"),
        GitAuthor("Chac Storm", "chac@rainmaker.org")
      )

      val messages = List(
        "feat(desktop): complete Phase 6 — The Pantheon of Rituals with dedicated Git workbenches",
        "perf(engine): pre-decode 48 frames into raw RGBA binary container",
        "ui(splash): add Grand Mayan Altar with real-time video stream and dynamic loading",
        "feat(dag): render feathered-serpent bezier curves for branch merges",
        "feat(blame): add Ahau lineage line-by-line heatmap gradient",
        "feat(reflog): time-travel altar with soft/mixed/hard reset dials",
        "feat(crucible): interactive rebase sequencer with pick/squash/drop",
        "feat(conflict): 3-way conflict altar with ours/theirs/result",
        "feat(radar): Ixchel DORA metrics telemetry bridge",
        "fix(linker): resolve symbol visibility in wgpu iced backend"
      )

      messages.zipWithIndex.take(limit).map { case (msg, idx) =>
        val timestamp = now - (idx.toLong * 3600000L * 4L)
        val author = authors(idx % authors.size)
        val sha = f"24fef${idx}%02x89b1c73a45e90d${idx}%02x"
        GitCommit(
          sha = CommitSha(sha),
          message = msg,
          author = author,
          timestamp = timestamp,
          additions = 120 + (idx * 37) % 450,
          deletions = 15 + (idx * 13) % 90,
          filesChanged = 1 + (idx * 3) % 12
        )
      }
    }

  override def fetchPullRequests(repoId: RepoId, limit: Int = 50): Task[List[GitPullRequest]] =
    Clock.currentTime(java.util.concurrent.TimeUnit.MILLISECONDS).map { now =>
      List(
        GitPullRequest(
          id = 101L,
          number = 1,
          title = "feat: Awaken Ixchel Git Analytics Engine & WebSocket Conduit",
          author = GitAuthor("Ahau Architect", "recluse@itzamna.io"),
          state = "open",
          createdAt = now - 7200000L,
          mergedAt = None,
          commentsCount = 4,
          reviewRounds = 2
        ),
        GitPullRequest(
          id = 100L,
          number = 2,
          title = "feat: The Pantheon of Rituals Dedicated Workbenches",
          author = GitAuthor("Ahau Architect", "recluse@itzamna.io"),
          state = "merged",
          createdAt = now - 86400000L,
          mergedAt = Some(now - 3600000L * 3L),
          commentsCount = 7,
          reviewRounds = 1
        ),
        GitPullRequest(
          id = 99L,
          number = 3,
          title = "perf: Raw RGBA Texture Streaming & Maya Altar Splash Screen",
          author = GitAuthor("Kukulkan Weave", "feathered@serpent.myth"),
          state = "merged",
          createdAt = now - (86400000L * 3L),
          mergedAt = Some(now - (86400000L * 2L)),
          commentsCount = 12,
          reviewRounds = 3
        )
      )
    }

object TelemetrySynthesizer:
  val live: ULayer[GitProvider] = ZLayer.succeed(new TelemetrySynthesizer)
