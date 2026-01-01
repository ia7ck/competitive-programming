package ABC424B

@main def main() =
  val Array(n, m, k) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val ab = (0 until k).map { _ =>
    val Array(a, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    (a, b)
  }

  case class S(score: Map[Int, Int], ans: Vector[Int])

  val acc = ab.foldLeft(S(Map(), Vector())) { case (S(score, ans), (a, _)) =>
    val v = score.getOrElse(a, 0) + 1
    val newScore = score.updated(a, v)
    val newAns = if v < m then ans else ans :+ a
    S(newScore, newAns)
  }

  println(acc.ans.mkString(" "))
