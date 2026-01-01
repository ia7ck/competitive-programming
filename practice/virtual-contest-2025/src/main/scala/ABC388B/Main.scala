package ABC388B

@main def main() =
  val Array(n, d) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val tl = (0 until n).map { _ =>
    val Array(t, l) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    (t, l)
  }

  for (k <- 1 to d) {
    val ans = tl.map((t, l) => t * (l + k)).max
    println(ans)
  }
