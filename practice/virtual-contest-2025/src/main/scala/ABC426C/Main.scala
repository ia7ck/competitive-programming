package ABC426C

@main def main() =
  val Array(n, q) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val xy = Array.fill(q) {
    val Array(x, y) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    (x, y)
  }

  val count = Array.tabulate(n + 1)(i => if i == 0 then 0 else 1)
  var minVersion = 1
  for ((x, y) <- xy) {
    val ans = if minVersion <= x then {
      val upgrade = (minVersion to x).map(count).sum
      for (i <- minVersion to x) {
        count(i) = 0
      }
      count(y) += upgrade
      minVersion = x + 1
      upgrade
    } else {
      0
    }
    println(ans)
  }
