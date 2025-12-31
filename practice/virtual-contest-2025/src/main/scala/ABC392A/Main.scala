package ABC392A

@main def main() =
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.permutations.exists(b => b(0) * b(1) == b(2))

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
