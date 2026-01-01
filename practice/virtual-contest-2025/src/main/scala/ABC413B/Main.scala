package ABC413B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val s = (0 until n).map(_ => scala.io.StdIn.readLine())

  val allPairs = for {
    i <- 0 until n
    j <- 0 until n
  } yield (i, j)
  val allStrings = allPairs.filter(_ != _).map((i, j) => s"${s(i)}${s(j)}")
  val ans = allStrings.distinct.length

  println(ans)
