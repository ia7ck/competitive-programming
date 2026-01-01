package ABC401B

@main def main() =
  val n = scala.io.StdIn.readInt()
  val queries = (0 until n).map(_ => scala.io.StdIn.readLine())

  case class S(login: Boolean, error: Int)

  val ans = queries.foldLeft(S(false, 0)) {
    case (S(false, e), "login")   => S(true, e)
    case (S(false, e), "private") => S(false, e + 1)
    case (S(true, e), "logout")   => S(false, e)
    case (s, _)                   => s
  }

  println(ans.error)
