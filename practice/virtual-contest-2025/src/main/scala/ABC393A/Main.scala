package ABC393A

@main def main() =
  val Array(s1, s2) = scala.io.StdIn.readLine().split(' ')

  val ans = (s1, s2) match {
    case ("sick", "sick") => 1
    case ("sick", "fine") => 2
    case ("fine", "sick") => 3
    case ("fine", "fine") => 4
    case _                => ???
  }

  println(ans)
