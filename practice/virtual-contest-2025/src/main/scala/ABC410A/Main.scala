package ABC410A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val k = scala.io.StdIn.readInt()

  val ans = a.count(k <= _)

  println(ans)
