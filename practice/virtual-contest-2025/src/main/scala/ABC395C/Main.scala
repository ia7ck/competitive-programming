package ABC395C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val pos = (0 until n).groupBy(i => a(i))
  val ans = pos
    .flatMap { (_, pos) =>
      pos.sliding(2).flatMap {
        case IndexedSeq(p1, p2) => Some(p2 - p1 + 1)
        case _                  => None
      }
    }
    .minOption
    .getOrElse(-1)

  println(ans)
