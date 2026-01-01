package ABC389C

@main def main() =
  val q = scala.io.StdIn.readInt()
  val queries = (0 until q).map { _ =>
    scala.io.StdIn.readLine() match {
      case s"1 ${rest}" => Query.Push(rest.toLong)
      case "2"          => Query.Pop
      case s"3 ${rest}" => Query.Query(rest.toInt - 1)
      case _            => ???
    }
  }

  case class S(prefixSum: Vector[Long], head: Int, ans: Vector[Long])

  val acc = queries.foldLeft(S(Vector(0), 0, Vector())) {
    case (S(prefixSum, head, ans), query) =>
      query match {
        case Query.Push(l) => {
          val last = prefixSum.last
          S(prefixSum :+ (last + l), head, ans)
        }
        case Query.Pop => {
          S(prefixSum, head + 1, ans)
        }
        case Query.Query(k) => {
          val pos = prefixSum(head + k) - prefixSum(head)
          S(prefixSum, head, ans :+ pos)
        }
      }
  }

  println(acc.ans.mkString("\n"))

enum Query:
  case Push(l: Long)
  case Pop
  case Query(k: Int)
