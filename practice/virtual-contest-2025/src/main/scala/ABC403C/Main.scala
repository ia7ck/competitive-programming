package ABC403C

@main def main() =
  val Array(n, m, q) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val queries = Array.fill(q) {
    scala.io.StdIn.readLine() match {
      case s"1 ${x} ${y}" => Query.Permit(x.toInt - 1, y.toInt - 1)
      case s"2 ${x}"      => Query.PermitAll(x.toInt - 1)
      case s"3 ${x} ${y}" => Query.View(x.toInt - 1, y.toInt - 1)
    }
  }

  case class S(
      permit: Vector[Set[Int]],
      permitAll: Vector[Boolean],
      ans: Vector[Boolean]
  )

  val acc = queries.foldLeft(
    S(Vector.fill(n)(Set()), Vector.fill(n)(false), Vector())
  ) { case (S(permit, permitAll, ans), query) =>
    query match {
      case Query.Permit(x, y) => {
        val p = permit(x).incl(y)
        S(permit.updated(x, p), permitAll, ans)
      }
      case Query.PermitAll(x) => {
        S(permit, permitAll.updated(x, true), ans)
      }
      case Query.View(x, y) => {
        val view = permitAll(x) || permit(x).contains(y)
        S(permit, permitAll, ans :+ view)
      }
    }
  }

  println(
    acc.ans
      .map {
        case true  => "Yes"
        case false => "No"
      }
      .mkString("\n")
  )

enum Query:
  case Permit(x: Int, y: Int)
  case PermitAll(x: Int)
  case View(x: Int, y: Int)
