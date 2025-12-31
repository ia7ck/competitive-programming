package ABC396A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.sliding(3).exists { case Array(x, y, z) => x == y && y == z }

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
