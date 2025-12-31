package ABC398A

@main def main() =
  val n = scala.io.StdIn.readInt()

  val ans = if n % 2 == 1 then {
    val pad = "-" * ((n - 1) / 2)
    s"${pad}=${pad}"
  } else {
    val pad = "-" * ((n - 2) / 2)
    s"${pad}==${pad}"
  }

  println(ans)
