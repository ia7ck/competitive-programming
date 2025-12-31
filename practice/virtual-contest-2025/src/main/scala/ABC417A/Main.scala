package ABC417A

@main def main() =
  val Array(n, a, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val s = scala.io.StdIn.readLine()

  val ans = s.drop(a).dropRight(b)

  println(ans)
