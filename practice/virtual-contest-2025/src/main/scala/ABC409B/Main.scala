package ABC409B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = (0 to n).filter(x => a.count(_ >= x) >= x).max

  println(ans)
