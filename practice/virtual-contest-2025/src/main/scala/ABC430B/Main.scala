package ABC430B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val s = (0 until n).map(_ => scala.io.StdIn.readLine())

  val grids = for {
    i <- 0 to n - m
    j <- 0 to n - m
  } yield s.slice(i, i + m).map(_.slice(j, j + m))
  val ans = grids.distinct.length

  println(ans)
