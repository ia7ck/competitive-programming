package ABC418A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val s = scala.io.StdIn.readLine()

  val ans = s.endsWith("tea")

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
