package ABC405A

@main def main() =
  val Array(r, x) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = if x == 1 then {
    1600 <= r && r <= 2999
  } else {
    1200 <= r && r <= 2399
  }

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
