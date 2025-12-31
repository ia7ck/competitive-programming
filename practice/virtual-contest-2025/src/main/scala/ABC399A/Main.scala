package ABC399A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val s = scala.io.StdIn.readLine()
  val t = scala.io.StdIn.readLine()

  val ans = s.zip(t).filter((s, t) => s != t).length

  println(ans)
