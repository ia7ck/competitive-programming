package ABC400B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toLong)

  val ans = (0L to m).foldLeft(Some(0L): Option[Long]) {
    case (None, _)      => None
    case (Some(acc), i) =>
      val newAcc = acc + n.pow(i)
      if newAcc > 1_000_000_000L then None
      else Some(newAcc)
  }

  ans match {
    case Some(ans) => println(ans)
    case None      => println("inf")
  }

extension (x: Long)
  def pow(y: Long): Long =
    assert(y >= 0)
    if (y == 0) 1
    else x * x.pow(y - 1)
