package ABC426B

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = s.find(c => s.count(_ == c) == 1).get

  println(ans)
