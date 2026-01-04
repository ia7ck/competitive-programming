package ABC439D

import scala.collection.mutable.HashMap

@main def main() =
  val scanner = FastScanner(java.lang.System.in)
  val n = scanner.nextInt()
  val a = Array.fill(n) { scanner.nextInt() }

  val ans = solve(a) + solve(a.reverse)

  println(ans)

def solve(a: Array[Int]): Long =
  var res = 0L
  val counter = HashMap.empty[Int, Long]
  for (a <- a) {
    if a % 5 == 0 then {
      val x = a / 5
      val i = counter.getOrElse(x * 7, 0L)
      val k = counter.getOrElse(x * 3, 0L)
      res += i * k
    }
    counter.updateWith(a) {
      case None    => Some(1)
      case Some(v) => Some(v + 1)
    }
  }
  res

class FastScanner(in: java.io.InputStream):
  import java.io.BufferedReader
  import java.util.StringTokenizer
  import java.io.InputStreamReader

  private val reader = new BufferedReader(new InputStreamReader(in))
  private var tokenizer: StringTokenizer = null

  def next(): String =
    while tokenizer == null || !tokenizer.hasMoreElements do
      val line = reader.readLine()
      if line == null then return null
      tokenizer = new StringTokenizer(line)
    tokenizer.nextToken()

  def nextInt(): Int = next().toInt
  def nextLong(): Long = next().toLong
  def nextDouble(): Double = next().toDouble
