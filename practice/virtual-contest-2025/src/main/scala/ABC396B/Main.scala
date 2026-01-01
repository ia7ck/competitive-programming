package ABC396B

import scala.collection.mutable.Stack

@main def main() =
  val q = scala.io.StdIn.readInt()
  val queries = (0 until q).map { _ =>
    val line = scala.io.StdIn.readLine()
    line.split(' ') match {
      case Array("1", rest) => Query.Pile(rest.toInt)
      case Array("2")       => Query.Remove
      case _                => ???
    }
  }

  val stack = Stack.fill(100)(0)
  for (q <- queries) {
    q match {
      case Query.Pile(x) => stack.push(x)
      case Query.Remove  => {
        val ans = stack.pop()
        println(ans)
      }
    }
  }

enum Query:
  case Pile(x: Int)
  case Remove
