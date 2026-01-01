package ABC412B

@main def main() =
  val s = scala.io.StdIn.readLine()
  val t = scala.io.StdIn.readLine()

  val ans = s.zipWithIndex.forall { (c, i) =>
    i == 0 || c.isLower || t.contains(s(i - 1))
  }

  if ans then println("Yes")
  else println("No")
