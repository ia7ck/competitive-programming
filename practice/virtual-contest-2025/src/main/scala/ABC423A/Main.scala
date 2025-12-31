package ABC423A

@main def main() =
  val Array(x, c) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = (0 to x by 1000).filter(y => y + y / 1000 * c <= x).max

  println(ans)
