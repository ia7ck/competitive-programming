package ABC427B

@main def main() =
  val n = scala.io.StdIn.readInt()

  val a = (1 to n).foldLeft(Vector(1L)) { (a, i) =>
    val y = a.map(f).sum
    a :+ y
  }
  val ans = a.last

  println(ans)

def f(x: Long) =
  x.toString().foldLeft(0)((acc, c) => acc + (c - '0'))
