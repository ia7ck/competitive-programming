package ABC413A

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.sum <= m

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
