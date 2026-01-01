package ABC397B

@main def main() =
  val s = scala.io.StdIn.readLine()

  val t = s.foldLeft(Vector[Char]()) {
    case (acc, 'i') if acc.length % 2 == 0 => acc :+ 'i'
    case (acc, 'i') if acc.length % 2 == 1 => acc :+ 'o' :+ 'i'
    case (acc, 'o') if acc.length % 2 == 0 => acc :+ 'i' :+ 'o'
    case (acc, 'o') if acc.length % 2 == 1 => acc :+ 'o'
    case _                                 => ???
  }
  val valid = if t.length % 2 == 0 then {
    t
  } else {
    t :+ 'o'
  }
  val ans = valid.length - s.length

  println(ans)
