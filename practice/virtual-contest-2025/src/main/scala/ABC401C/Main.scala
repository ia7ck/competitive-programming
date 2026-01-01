package ABC401C

@main def main() =
  val Array(n, k) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  case class S(a: Vector[Long], prefixSum: Vector[Long])

  val m = 1_000_000_000
  val acc = (0 to n).foldLeft(S(Vector(), Vector(0))) {
    case (S(a, prefixSum), i) => {
      val last = prefixSum.last
      val v = if i < k then {
        1
      } else {
        ((last - prefixSum(i - k)) % m + m) % m
      }
      S(a :+ v, prefixSum :+ (last + v) % m)
    }
  }

  println(acc.a.last)
