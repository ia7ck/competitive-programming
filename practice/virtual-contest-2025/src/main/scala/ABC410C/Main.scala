package ABC410C

@main def main() =
  val Array(n, q) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val queries = Array.fill(q) {
    scala.io.StdIn.readLine() match {
      case s"1 ${p} ${x}" => Query.Update(p.toInt - 1, x.toInt)
      case s"2 ${p}"      => Query.Print(p.toInt - 1)
      case s"3 ${k}"      => Query.Rotate(k.toInt)
    }
  }

  case class S(a: Vector[Int], offset: Int, ans: Vector[Int])

  val acc = queries.foldLeft(S(Vector.tabulate(n)(i => i + 1), 0, Vector())) {
    case (S(a, offset, ans), query) =>
      query match {
        case Query.Update(p, x) => {
          val i = (p + offset) % n
          S(a.updated(i, x), offset, ans)
        }
        case Query.Print(p) => {
          val i = (p + offset) % n
          S(a, offset, ans :+ a(i))
        }
        case Query.Rotate(k) => S(a, (offset + k) % n, ans)
      }
  }

  println(acc.ans.mkString("\n"))

enum Query:
  case Update(p: Int, x: Int)
  case Print(p: Int)
  case Rotate(k: Int)
