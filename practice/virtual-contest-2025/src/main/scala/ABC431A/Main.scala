package ABC431A

@main def main() =
  val Array(h, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = (h - b).max(0)

  println(ans)
