package ABC392C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val p = scala.io.StdIn.readLine().split(' ').map(_.toInt - 1)
  val q = scala.io.StdIn.readLine().split(' ').map(_.toInt - 1)

  val invQ = q.zipWithIndex.foldLeft(Vector.fill(n)(-1)) { case (acc, (x, i)) =>
    acc.updated(x, i)
  }
  val ans = (0 until n).map(i => q(p(invQ(i))))

  println(ans.map(_ + 1).mkString(" "))
