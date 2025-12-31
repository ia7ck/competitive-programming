package ABC387A

@main def main() =
  val line = scala.io.StdIn.readLine()
  val Array(a, b) = line.split(' ').map(_.toInt)

  val ans = (a + b) * (a + b)
  println(ans)
