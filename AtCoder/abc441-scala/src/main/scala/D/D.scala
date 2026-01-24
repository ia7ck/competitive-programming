package D

import scala.collection.mutable.ArrayBuffer

@main def main =
  val fs = FastScanner(java.lang.System.in)
  val n = fs.nextInt()
  val m = fs.nextInt()
  val l = fs.nextInt()
  val s = fs.nextLong()
  val t = fs.nextLong()
  val edges = List.fill(m) {
    val u = fs.nextInt()
    val v = fs.nextInt()
    val c = fs.nextLong()
    (u - 1, v - 1, c)
  }

  val g = Array.fill(n)(ArrayBuffer.empty[(to: Int, cost: Long)])
  for ((u, v, c) <- edges) {
    g(u).addOne((v, c))
  }

  var acc = ArrayBuffer((v = 0, sum = 0L))
  for (_ <- 1 to l) {
    val next = ArrayBuffer.empty[(v: Int, sum: Long)]
    for ((v, sum) <- acc) {
      for ((to, cost) <- g(v)) {
        next.addOne((to, sum + cost))
      }
    }
    acc = next
  }
  val ans =
    acc
      .flatMap {
        case (v, sum) if s <= sum && sum <= t => Some(v)
        case _                                => None
      }
      .distinct
      .sorted
      .map(_ + 1)
      .mkString(" ")

  println(ans)

class FastScanner(in: java.io.InputStream):
  import java.io.BufferedReader
  import java.util.StringTokenizer
  import java.io.InputStreamReader

  private val reader = new BufferedReader(new InputStreamReader(in))
  private var tokenizer: StringTokenizer = null

  def next(): String =
    while tokenizer == null || !tokenizer.hasMoreElements do
      val line = reader.readLine()
      if line == null then sys.error("EOF")
      tokenizer = new StringTokenizer(line)
    tokenizer.nextToken()

  def nextInt(): Int = next().toInt
  def nextLong(): Long = next().toLong
  def nextDouble(): Double = next().toDouble
