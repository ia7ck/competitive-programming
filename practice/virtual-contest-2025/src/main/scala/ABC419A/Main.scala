package ABC419A

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = s match {
    case "red"   => "SSS"
    case "blue"  => "FFF"
    case "green" => "MMM"
    case _       => "Unknown"
  }

  println(ans)
