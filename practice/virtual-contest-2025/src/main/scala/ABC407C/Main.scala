package ABC407C

@main def main() =
  val s = scala.io.StdIn.readLine().map(_.asDigit)

  case class S(a: Int, b: Int)

  val acc = s.foldRight(S(0, 0)) {
    case (d, S(a, b)) => {
      val pushA = 1
      val pushB = ((d - b) % 10 + 10) % 10
      S(a + pushA, b + pushB)
    }
  }
  val ans = acc.a + acc.b

  println(ans)
