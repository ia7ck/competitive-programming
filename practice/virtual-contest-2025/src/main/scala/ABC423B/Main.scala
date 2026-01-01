package ABC423B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val l = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val i = l.indexOf(1)
  val j = l.lastIndexOf(1)
  val ans = if i == j then {
    0
  } else {
    j - i
  }

  println(ans)
