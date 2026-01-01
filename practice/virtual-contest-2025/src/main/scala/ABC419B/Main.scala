package ABC419B

import scala.collection.mutable.PriorityQueue

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

  val pq = PriorityQueue()(using Ordering[Int].reverse)
  for (q <- queries) {
    q match {
      case Query.Push(x) => pq.enqueue(x)
      case Query.Pop     => {
        val ans = pq.dequeue()
        println(ans)
      }
    }
  }

enum Query:
  case Push(x: Int)
  case Pop
