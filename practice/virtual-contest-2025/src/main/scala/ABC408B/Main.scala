package ABC408B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.sorted.distinct

  println(ans.length)
  println(ans.mkString(" "))
