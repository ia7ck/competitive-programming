package ABC400A

@main def main() =
  val a = scala.io.StdIn.readInt()

  val ans = if 400 % a == 0 then {
    400 / a
  } else {
    -1
  }

  println(ans)
