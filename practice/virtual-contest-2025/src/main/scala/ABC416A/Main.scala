package ABC416A

@main def main() =
  val Array(n, l, r) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val s = scala.io.StdIn.readLine()

  val ans = s.substring(l - 1, r).forall(_ == 'o')

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
