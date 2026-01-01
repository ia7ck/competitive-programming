package ABC421B

@main def main() =
  val Array(x, y) = scala.io.StdIn.readLine().split(' ').map(_.toLong)

  val a = (3 to 10).foldLeft(Vector(x, y)) { (acc, _) =>
    val n = acc.length
    val y = f(acc(n - 1) + acc(n - 2))
    acc :+ y
  }
  val ans = a.last

  println(ans)

def f(x: Long) =
  x.toString.reverse.toLong
