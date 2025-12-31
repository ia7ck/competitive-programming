package ABC424A

@main def main() =
  val Array(a, b, c) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a == b || b == c || c == a

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
