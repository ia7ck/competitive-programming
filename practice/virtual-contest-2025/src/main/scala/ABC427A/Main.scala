package ABC427A

@main def main() =
  val s = scala.io.StdIn.readLine()

  val n = s.length
  val ans = s"${s.take(n / 2)}${s.takeRight(n / 2)}"

  println(ans)
