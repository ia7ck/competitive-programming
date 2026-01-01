package ABC414B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val cl = (0 until n).map { _ =>
    val line = scala.io.StdIn.readLine()
    val Array(c, l) = line.split(' ')
    (c, l.toLong)
  }

  val ans = cl.foldLeft(Some(""): Option[String]) {
    case (None, _)           => None
    case (Some(acc), (c, l)) => {
      val newLen = acc.length + l
      if newLen > 100 then {
        None
      } else {
        Some(s"${acc}${c * l.toInt}")
      }
    }
  }

  println(ans.getOrElse("Too Long"))
