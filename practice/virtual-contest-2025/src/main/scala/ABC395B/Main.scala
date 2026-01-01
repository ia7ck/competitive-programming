package ABC395B

@main def main() =
  val n = scala.io.StdIn.readInt()

  val ans = Array.fill(n, n)('.')
  for (k <- 0 to n / 2) {
    if k % 2 == 0 then {
      for (i <- k until n - k) {
        ans(i)(k) = '#'
        ans(i)(n - k - 1) = '#'
        ans(k)(i) = '#'
        ans(n - k - 1)(i) = '#'
      }
    }
  }

  println(ans.map(_.mkString).mkString("\n"))
