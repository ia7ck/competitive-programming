package ABC407A

@main def main() =
  val Array(a, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val c = a / b
  val ans =
    Array(c, c + 1).minBy(c => (c.toDouble - (a.toDouble / b.toDouble)).abs)

  println(ans)
