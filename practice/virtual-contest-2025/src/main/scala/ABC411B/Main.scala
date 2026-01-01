package ABC411B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val d = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  for (d <- d.tails) {
    val cumsum = d.scanLeft(0)(_ + _).drop(1)
    println(cumsum.mkString(" "))
  }
