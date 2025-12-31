package ABC403A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = (0 until n by 2).map(i => a(i)).sum

  println(ans)
