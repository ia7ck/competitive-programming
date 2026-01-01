package ABC387B

@main def main() =
  val x = scala.io.StdIn.readInt()

  val xs = for {
    i <- 1 to 9
    j <- 1 to 9
  } yield i * j
  val ans = xs.filter(_ != x).sum

  println(ans)
