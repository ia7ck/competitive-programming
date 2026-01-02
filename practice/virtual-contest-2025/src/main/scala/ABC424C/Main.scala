package ABC424C

import scala.collection.mutable.Queue
import scala.collection.mutable.ArrayBuffer
import java.io.BufferedReader
import java.util.StringTokenizer
import java.io.InputStreamReader

@main def main() =
  val scanner = FastScanner(java.lang.System.in)
  val n = scanner.nextInt()
  val ab = List.fill(n) {
    val a = scanner.nextInt()
    val b = scanner.nextInt()
    (a, b)
  }

  val (starts, edges) = ab.zipWithIndex.partitionMap {
    case ((0, 0), i) => Left(i + 1)
    case ((a, b), i) => Right(List((a, i + 1), (b, i + 1)))
  }

  val graph = Array.fill(n + 1) { ArrayBuffer.empty[Int] }
  for {
    es <- edges
    (from, to) <- es
  } {
    graph(from) += to
  }

  val visited = Array.fill(n + 1)(false)
  val queue = Queue.empty[Int]
  for (s <- starts) {
    visited(s) = true
    queue.enqueue(s)
  }
  while (!queue.isEmpty) {
    val x = queue.dequeue()
    for { y <- graph(x) if !visited(y) } {
      visited(y) = true
      queue.enqueue(y)
    }
  }

  println(visited.count(_ == true))

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
