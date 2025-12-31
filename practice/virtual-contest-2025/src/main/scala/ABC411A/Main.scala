package ABC411A

@main def main() =
  val p = scala.io.StdIn.readLine()
  val l = scala.io.StdIn.readInt()

  val ans = p.length >= l

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
