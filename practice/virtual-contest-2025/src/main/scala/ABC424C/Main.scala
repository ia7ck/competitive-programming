package ABC424C

import scala.collection.mutable.Queue

@main def main() =
  val n = scala.io.StdIn.readInt()
  val ab = List.fill(n) {
    val Array(a, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    (a, b)
  }

  val edges = for {
    ((a, b), i) <- ab.zipWithIndex
    if (a, b) != (0, 0)
    e <- List((a, i + 1), (b, i + 1))
  } yield e

  val graph = edges
    .groupMap((from, _) => from)((_, to) => to)
    // toArray
    .foldLeft(Array.fill(n + 1) { List.empty[Int] }) {
      case (graph, (from, to)) =>
        graph(from) = to
        graph
    }

  val starts = ab.zipWithIndex
    .filter { case ((a, b), _) => (a, b) == (0, 0) }
    .map((_, i) => i + 1)
    .toList
  val visited = Array.fill(n + 1)(false)
  val queue = Queue.empty[Int]
  for (s <- starts) {
    visited(s) = true
    queue.enqueue(s)
  }
  while (!queue.isEmpty) {
    val x = queue.dequeue()
    for (y <- graph(x)) {
      if !visited(y) then {
        visited(y) = true
        queue.enqueue(y)
      }
    }
  }

  println(visited.count(_ == true))
