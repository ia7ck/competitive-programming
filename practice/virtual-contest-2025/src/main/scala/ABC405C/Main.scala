package ABC405C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toLong)

  val suffixSum = a.scanRight(0L)(_ + _)
  val `a(i) * sum(a(j))` = a.zipWithIndex.map((a, i) => a * suffixSum(i + 1))
  val ans = `a(i) * sum(a(j))`.sum

  println(ans)
