package ABC414A

@main def main() =
  val Array(n, l, r) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val xy = (0 until n).map { _ =>
    val Array(x, y) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    (x, y)
  }

  val ans = xy.count((x, y) => x <= l && r <= y)

  println(ans)
