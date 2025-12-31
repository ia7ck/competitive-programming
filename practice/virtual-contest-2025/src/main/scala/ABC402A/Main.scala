package ABC402A

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = s.filter(_.isUpper).mkString

  println(ans)
