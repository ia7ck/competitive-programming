package ABC394C

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = s.toArray
  for (i <- (s.length - 2) to 0 by -1) {
    if (ans(i) == 'W' && ans(i + 1) == 'A') then {
      ans(i) = 'A'
      ans(i + 1) = 'C'
    }
  }

  println(ans.mkString(""))
