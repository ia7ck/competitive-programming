package ABC393B

@main def main() =
  val s = scala.io.StdIn.readLine()

  val triple = for {
    i <- 0 until s.length
    j <- i + 1 until s.length
    k <- j + 1 until s.length
  } yield (i, j, k)
  val ans = triple.count((i, j, k) =>
    j - i == k - j && s(i) == 'A' && s(j) == 'B' && s(k) == 'C'
  )

  println(ans)
