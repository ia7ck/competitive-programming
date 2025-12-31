package ABC415A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val x = scala.io.StdIn.readInt()

  val ans = a.contains(x)

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
