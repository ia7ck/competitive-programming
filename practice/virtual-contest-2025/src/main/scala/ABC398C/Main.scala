package ABC398C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val group = (0 until n).groupBy(i => a(i))
  val ans = group
    .filter((_, v) => v.length == 1)
    .maxByOption((k, _) => k)
    .map((_, v) => v.head + 1)
    .getOrElse(-1)

  println(ans)
