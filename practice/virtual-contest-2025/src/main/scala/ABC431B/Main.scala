package ABC431B

@main def main() =
  val x = scala.io.StdIn.readInt()
  val n = scala.io.StdIn.readInt()
  val w = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val q = scala.io.StdIn.readInt()
  val queries = (0 until q).map(_ => scala.io.StdIn.readInt() - 1)

  case class S(ans: Vector[Int], attach: Vector[Boolean])

  val acc = queries.foldLeft(S(Vector(x), Vector.fill(n)(false))) {
    case (S(ans, attach), p) =>
      val a = attach(p)
      val newX = ans.last + (if a then -w(p) else w(p))

      S(ans :+ newX, attach.updated(p, !a))
  }

  println(acc.ans.tail.mkString("\n"))
