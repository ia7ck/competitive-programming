package ABC391
package ABC391C

@main def main() =
  val Array(n, q) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val queries = Array.fill(q) {
    scala.io.StdIn.readLine() match {
      case s"1 ${p} ${h}" => Query.Move(p.toInt - 1, h.toInt - 1)
      case "2"            => Query.Query
      case _              => ???
    }
  }

  case class S(
      p2h: Vector[Int],
      h2ps: Vector[Set[Int]],
      // pigeions >= 2
      count: Int,
      ans: Vector[Int]
  )

  val acc = queries.foldLeft(
    S(
      p2h = (0 until n).toVector,
      h2ps = Vector.tabulate(n)(i => Set(i)),
      count = 0,
      ans = Vector()
    )
  ) { case (S(p2h, h2ps, count, ans), query) =>
    query match {
      case Query.Move(p, h) => {
        val oldH = p2h(p)
        val oldP = h2ps(oldH)
        val newP = h2ps(h)
        val h2psNew = h2ps.updated(oldH, oldP.excl(p)).updated(h, newP.incl(p))
        val p2hNew = p2h.updated(p, h)
        val countNew = (h2psNew(oldH).size, h2psNew(h).size) match {
          case (1, 2) => count
          case (1, _) => count - 1
          case (_, 2) => count + 1
          case _      => count
        }
        S(p2hNew, h2psNew, countNew, ans)
      }
      case Query.Query => {
        val newAns = ans :+ count
        S(p2h, h2ps, count, newAns)
      }
    }
  }

  println(acc.ans.mkString("\n"))

enum Query:
  case Move(p: Int, h: Int)
  case Query
