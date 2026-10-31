package ixchel.provider

import ixchel.domain.*
import zio.*

trait GitProvider:
  def providerKind: ProviderKind
  def fetchRepoMetadata(repoId: RepoId): Task[RepoMetadata]
  def fetchCommits(repoId: RepoId, limit: Int = 100): Task[List[GitCommit]]
  def fetchPullRequests(repoId: RepoId, limit: Int = 50): Task[List[GitPullRequest]]
