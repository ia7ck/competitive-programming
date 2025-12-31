package ABC421A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val s = (0 until n).map(_ => scala.io.StdIn.readLine()).toArray
  val Array(x_, y) = scala.io.StdIn.readLine().split(' ')
  val x = x_.toInt

  if s(x - 1) == y then {
    println("Yes")
  } else {
    println("No")
  }
