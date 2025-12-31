package ABC409A

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine()
  val b = scala.io.StdIn.readLine()

  val ans = a.zip(b).exists((a, b) => a == 'o' && b == 'o')

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
