package ABC430A

@main def main() =
  val Array(a, b, c, d) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a <= c && b > d

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
