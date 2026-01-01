package ABC390B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toLong)

  val ans = a.sliding(2).forall { case Array(p, q) =>
    // a(1)/a(0) == q/p
    a(1) * p == a(0) * q
  }

  if ans then println("Yes")
  else println("No")
