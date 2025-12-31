package ABC401A

@main def main() =
  val s = scala.io.StdIn.readInt()

  val ans = if 200 <= s && s <= 299 then {
    "Success"
  } else {
    "Failure"
  }

  println(ans)
