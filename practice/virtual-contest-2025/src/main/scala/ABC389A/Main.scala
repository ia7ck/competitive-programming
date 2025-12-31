package ABC389A

@main def main() =
  val line = scala.io.StdIn.readLine()

  val (a, b) = line match {
    case s"${a}x${b}" => (a.toInt, b.toInt)
    case _            => ???
  }

  val ans = a * b
  println(ans)
