package ABC428A

@main def main() =
  val Array(s, a, b, x) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  var ans = 0
  var t = 0
  while (t < x) {
    val newT = (t + a).min(x)
    ans += (newT - t) * s
    t = newT + b
  }

  println(ans)
