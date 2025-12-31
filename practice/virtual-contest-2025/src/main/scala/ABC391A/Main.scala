package ABC391A

@main def main() =
  val dir = scala.io.StdIn.readLine()

  val ans = dir match {
    case "N"  => "S"
    case "E"  => "W"
    case "W"  => "E"
    case "S"  => "N"
    case "NE" => "SW"
    case "NW" => "SE"
    case "SE" => "NW"
    case "SW" => "NE"
    case _    => ???
  }

  println(ans)
