package ABC425A

@main def main() =
  val n = scala.io.StdIn.readInt()

  val ans = (1 to n).map(i => (-1).pow(i) * i.pow(3)).sum

  println(ans)

extension (x: Int)
  def pow(y: Int): Int =
    assert(y >= 0)
    if (y == 0) 1
    else x * x.pow(y - 1)
