package ABC392B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = (1 to n).filter(i => !a.contains(i)).toArray
  println(ans.length)
  println(ans.mkString(" "))
