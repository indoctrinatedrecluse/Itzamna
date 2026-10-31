package ixchel.domain

import zio.json.*

// --- Opaque Types for Domain Safety ---

opaque type AccountId = String
object AccountId:
  def apply(value: String): AccountId = value
  extension (id: AccountId) def value: String = id
  given JsonEncoder[AccountId] = JsonEncoder.string.contramap(_.value)
  given JsonDecoder[AccountId] = JsonDecoder.string.map(AccountId(_))

opaque type RepoId = String
object RepoId:
  def apply(value: String): RepoId = value
  extension (id: RepoId) def value: String = id
  given JsonEncoder[RepoId] = JsonEncoder.string.contramap(_.value)
  given JsonDecoder[RepoId] = JsonDecoder.string.map(RepoId(_))

opaque type CommitSha = String
object CommitSha:
  def apply(value: String): CommitSha = value
  extension (sha: CommitSha)
    def value: String = sha
    def short: String = if sha.length > 7 then sha.substring(0, 7) else sha
  given JsonEncoder[CommitSha] = JsonEncoder.string.contramap(_.value)
  given JsonDecoder[CommitSha] = JsonDecoder.string.map(CommitSha(_))

// --- Provider & Git Entities ---

enum ProviderKind:
  case GitHub, GitLab, Bitbucket

object ProviderKind:
  given JsonCodec[ProviderKind] = DeriveJsonCodec.gen[ProviderKind]

final case class GitAuthor(
    name: String,
    email: String,
    avatarUrl: Option[String] = None
)

object GitAuthor:
  given JsonCodec[GitAuthor] = DeriveJsonCodec.gen[GitAuthor]

final case class GitCommit(
    sha: CommitSha,
    message: String,
    author: GitAuthor,
    timestamp: Long,
    additions: Int,
    deletions: Int,
    filesChanged: Int
)

object GitCommit:
  given JsonCodec[GitCommit] = DeriveJsonCodec.gen[GitCommit]

final case class GitPullRequest(
    id: Long,
    number: Int,
    title: String,
    author: GitAuthor,
    state: String, // "open", "merged", "closed"
    createdAt: Long,
    mergedAt: Option[Long],
    commentsCount: Int,
    reviewRounds: Int
)

object GitPullRequest:
  given JsonCodec[GitPullRequest] = DeriveJsonCodec.gen[GitPullRequest]

final case class RepoMetadata(
    id: RepoId,
    name: String,
    owner: String,
    defaultBranch: String,
    provider: ProviderKind,
    isPrivate: Boolean,
    url: String
)

object RepoMetadata:
  given JsonCodec[RepoMetadata] = DeriveJsonCodec.gen[RepoMetadata]
