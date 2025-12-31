package ABC420A

@main def main() =
  val Array(x, y) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val z = x + y
  val ans = if z <= 12 then {
    z
  } else {
    z - 12
  }

  println(ans)
