package E

import scala.collection.mutable.PriorityQueue
import scala.collection.mutable.ArrayBuffer
import scala.collection.mutable.HashSet

@main def main =
  val Array(n, k, x) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toLong)

  a.sortInPlace()(using Ordering[Long].reverse)

  def hash(b: Iterable[Int]): (Long, Long) =
    // b.foldLeft((0L, 0L)) { case ((h1, h2), b) =>
    //   (
    //     (h1 * 12345 + (b + 1)) % 998244353,
    //     (h2 * 56789 + (b + 1)) % 1000000007
    //   )
    // }

    var h1 = 0L
    var h2 = 0L
    for (b <- b) {
      h1 = (h1 * 12345 + (b + 1)) % 998244353
      h2 = (h2 * 56789 + (b + 1)) % 1000000007
    }
    (h1, h2)

  def score(b: Iterable[Int]): Long =
    a.iterator.zip(b).map((a, b) => a * b).sum

  val ans = ArrayBuffer.empty[Long]
  val initial = k +: Array.fill(n - 1)(0)
  val seen = HashSet(hash(initial))
  val pq = PriorityQueue((initial, score(initial)))(using Ordering.by(_._2))
  while (ans.length < x) {
    val (b, c) = pq.dequeue
    ans += c
    for (
      i <- 0 until (n - 1)
      if b(i) > 0
    ) {
      val nextB = b.clone()
      nextB(i) -= 1
      nextB(i + 1) += 1
      val h = hash(nextB)
      if (seen.add(h)) {
        pq.enqueue((nextB, score(nextB)))
      }
    }
  }
  println(ans.mkString("\n"))
