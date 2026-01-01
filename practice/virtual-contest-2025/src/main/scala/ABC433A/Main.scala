package ABC433A

@main def main() =
  val Array(x, y, z) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  var t = 0
  while (x + t > (y + t) * z) {
    t += 1
  }
  val ans = x + t == (y + t) * z

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
