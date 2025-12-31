package ABC394A

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = s.filter(_ == '2').mkString

  println(ans)
