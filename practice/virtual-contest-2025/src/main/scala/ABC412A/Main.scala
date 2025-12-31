package ABC412A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val ab = (0 until n).map { _ =>
    val Array(a, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    (a, b)
  }

  val ans = ab.count((a, b) => a < b)

  println(ans)
