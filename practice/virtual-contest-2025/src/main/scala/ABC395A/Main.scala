package ABC395A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.sliding(2).forall { case Array(x, y) => x < y }

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
