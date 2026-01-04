package ABC439E

import scala.collection.Searching.Found
import scala.collection.Searching.InsertionPoint

@main def main =
  val scanner = FastScanner(java.lang.System.in)
  val n = scanner.nextInt()
  val ab = List.fill(n) {
    val a = scanner.nextInt()
    val b = scanner.nextInt()
    (a, b)
  }

  val dp = Array.fill(n + 1)(Int.MaxValue)
  dp(0) = -1
  for ((_, b) <- ab.sortBy((a, b) => (a, -b))) {
    dp.search(b - 1) match {
      case Found(foundIndex) => {
        dp(foundIndex + 1) = dp(foundIndex + 1).min(b)
      }
      case InsertionPoint(insertionPoint) => {
        dp(insertionPoint) = b
      }
    }
  }
  val ans = dp.lastIndexWhere(_ < Int.MaxValue)

  println(ans)

import java.io.BufferedReader
import java.util.StringTokenizer
import java.io.InputStreamReader

class FastScanner(in: java.io.InputStream):
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
