package ABC402B

import scala.collection.mutable.Queue

@main def main() =
  val q = scala.io.StdIn.readInt()
  val queries = (0 until q).map { _ =>
    val line = scala.io.StdIn.readLine()
    line.split(' ') match {
      case Array("1", rest) => Query.Push(rest.toInt)
      case Array("2")       => Query.Pop
      case _                => ???
    }
  }

  val queue = Queue[Int]()
  for (q <- queries) {
    q match {
      case Query.Push(x) => queue.enqueue(x)
      case Query.Pop     => {
        val ans = queue.dequeue()
        println(ans)
      }
    }
  }

enum Query:
  case Push(x: Int)
  case Pop
