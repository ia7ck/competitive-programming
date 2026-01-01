package ABC398B

@main def main() =
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.combinations(5).exists { a =>
    val x = a.min
    val y = a.max
    a.sorted match {
      case Array(`x`, `x`, `x`, `y`, `y`) if x < y => true
      case Array(`x`, `x`, `y`, `y`, `y`) if x < y => true
      case _                                       => false
    }
  }

  if ans then println("Yes")
  else println("No")
