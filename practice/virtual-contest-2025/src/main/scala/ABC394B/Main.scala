package ABC394B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val s = (0 until n).map(_ => scala.io.StdIn.readLine()).toList

  val ans = s.sortBy(_.length).mkString

  println(ans)
