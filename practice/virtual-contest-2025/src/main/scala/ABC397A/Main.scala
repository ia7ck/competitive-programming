package ABC397A

@main def main() =
  val x = scala.io.StdIn.readDouble()

  val ans = if x >= 38.0 then {
    1
  } else if x >= 37.5 then {
    2
  } else {
    3
  }

  println(ans)
