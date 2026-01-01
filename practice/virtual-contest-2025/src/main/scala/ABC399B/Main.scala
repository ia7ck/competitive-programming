package ABC399B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val p = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  for (x <- p) {
    val rank = p.count(_ > x)
    println(rank + 1)
  }
