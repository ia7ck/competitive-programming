package ABC417C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  case class S(counter: Map[Int, Long], ans: Long)

  val acc = a.zipWithIndex.foldLeft(S(Map(), 0)) {
    case (S(counter, ans), (a, j)) => {
      val newAns = ans + counter.getOrElse((j + 1) - a, 0L)
      val key = (j + 1) + a
      val newCounter = counter.updated(key, counter.getOrElse(key, 0L) + 1)
      S(newCounter, newAns)
    }
  }

  println(acc.ans)
