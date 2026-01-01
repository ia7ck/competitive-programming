package ABC419C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val rc = Array.fill(n) {
    val Array(r, c) = scala.io.StdIn.readLine().split(' ').map(_.toLong)
    (r, c)
  }

  val rMin = rc.map((r, _) => r).min
  val rMax = rc.map((r, _) => r).max
  val cMin = rc.map((_, c) => c).min
  val cMax = rc.map((_, c) => c).max
  val r0 = (rMin + rMax) / 2
  val c0 = (cMin + cMax) / 2
  val ans = rc.map((r, c) => (r - r0).max(c - c0)).max

  println(ans)
