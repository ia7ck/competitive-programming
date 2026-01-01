package ABC406B

@main def main() =
  val Array(n, k) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toLong)

  val ans = a.foldLeft(1L) { (acc, x) =>
    val newAcc = BigInt(acc) * BigInt(x)
    if newAcc.toString.length >= k + 1 then {
      1L
    } else {
      newAcc.toLong
    }
  }

  println(ans)
