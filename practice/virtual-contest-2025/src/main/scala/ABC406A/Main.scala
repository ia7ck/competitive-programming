package ABC406A

@main def main() =
  val Array(a, b, c, d) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ord = Ordering[(Int, Int)]
  val ans = ord.lt((c, d), (a, b))

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
