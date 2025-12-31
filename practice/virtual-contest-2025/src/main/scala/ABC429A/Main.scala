package ABC429A

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  for (i <- 1 to n) {
    if i <= m then {
      println("OK")
    } else {
      println("Too Many Requests")
    }
  }
