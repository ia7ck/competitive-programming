package ABC388C

import scala.collection.Searching.Found
import scala.collection.Searching.InsertionPoint

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.foldLeft(0L) { (acc, x) =>
    var low = -1
    var high = a.length
    // a(low) < x * 2, a(high) >= x * 2
    while (low + 1 < high) {
      val mid = (low + high) / 2
      if a(mid) < x * 2 then low = mid else high = mid
    }
    acc + (n - high).toLong
  }

  println(ans)
