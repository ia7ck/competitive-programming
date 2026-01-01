package ABC415B

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = (0 until s.length)
    .filter(i => s(i) == '#')
    .grouped(2)
    .map(g => s"${g(0) + 1},${g(1) + 1}")
    .mkString("\n")

  println(ans)
