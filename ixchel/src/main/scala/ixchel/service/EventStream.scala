package ixchel.service

import ixchel.domain.*
import zio.*

trait EventStream:
  def broadcast(event: TelemetryEvent): UIO[Boolean]
  def subscribe: ZIO[Scope, Nothing, Dequeue[TelemetryEvent]]
  def activeClients: UIO[Int]
  def incrementClients: UIO[Int]
  def decrementClients: UIO[Int]

final class EventStreamLive(hub: Hub[TelemetryEvent], clientCount: Ref[Int]) extends EventStream:
  override def broadcast(event: TelemetryEvent): UIO[Boolean] =
    hub.publish(event)

  override def subscribe: ZIO[Scope, Nothing, Dequeue[TelemetryEvent]] =
    hub.subscribe

  override def activeClients: UIO[Int] =
    clientCount.get

  override def incrementClients: UIO[Int] =
    clientCount.updateAndGet(_ + 1)

  override def decrementClients: UIO[Int] =
    clientCount.updateAndGet(c => math.max(0, c - 1))

object EventStreamLive:
  val layer: ULayer[EventStream] =
    ZLayer {
      for
        hub   <- Hub.sliding[TelemetryEvent](256)
        count <- Ref.make(0)
      yield new EventStreamLive(hub, count)
    }
