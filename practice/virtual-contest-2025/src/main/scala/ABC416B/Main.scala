package ABC416B

@main def main() =
  val s = scala.io.StdIn.readLine()

  // ?: sentinel
  val t = s"${s}?"
    .split('#')
    .map {
      case ""          => ""
      case "?"         => "?"
      case s".${rest}" => s"o${rest}"
      case _           => ???
    }
    .mkString("#")

  println(t.stripSuffix("?"))
