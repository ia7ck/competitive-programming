package ABC422A

@main def main() =
  val s = scala.io.StdIn.readLine()

  val ans = s match {
    case s"${world}-8"        => s"${world.toInt + 1}-1"
    case s"${world}-${stage}" => s"${world}-${stage.toInt + 1}"
    case _                    => ???
  }

  println(ans)
