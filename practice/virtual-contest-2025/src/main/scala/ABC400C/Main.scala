package ABC400C

@main def main() =
  val n = scala.io.StdIn.readLong()

  val count = (1 until 64).map { a =>
    val `b^2 Max` = n / 2L.pow(a)
    val bMax = isqrt(`b^2 Max`)
    // 1,2,...,bMax のうち奇数の個数
    (bMax + 1) / 2
  }
  val ans = count.sum

  println(ans)

def isqrt(x: Long): Long =
  val y = math.sqrt(x.toDouble).toLong
  if y * y <= x then y else y - 1

extension (x: Long)
  def pow(y: Long): Long =
    assert(y >= 0)
    if (y == 0) 1
    else x * x.pow(y - 1)
