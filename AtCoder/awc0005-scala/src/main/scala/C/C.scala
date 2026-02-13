package C

import scala.collection.mutable.PriorityQueue

@main def main =
  val fs = FastScanner(java.lang.System.in)
  val n = fs.nextInt()
  val k = fs.nextInt()
  val a = Array.fill(n) { fs.nextLong() }

  val b = a.clone()
  val seen = Array.fill(n)(false)
  val queue = PriorityQueue.from(a.zipWithIndex)
  for (_ <- 0 until n) {
    while {
      val (_, i) = queue.head
      seen(i)
    } do {
      queue.dequeue()
    }

    val (x, i) = queue.dequeue()
    assert(!seen(i))
    b(i) = x
    seen(i) = true

    if (i >= 1) {
      queue.enqueue((x - k, i - 1))
    }
    if (i + 1 < n) {
      queue.enqueue((x - k, i + 1))
    }
  }

  val ans = a.zip(b).map((a, b) => b - a).sum
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
