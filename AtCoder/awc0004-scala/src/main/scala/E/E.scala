package E

@main def main =
  val fs = FastScanner(java.lang.System.in)
  val n = fs.nextInt()
  val k = fs.nextLong()
  val a = Array.fill(n) { fs.nextLong() }

  case class State(prefixSum: Long, counter: Map[Long, Long], ans: Long)

  val acc = a.foldLeft(State(0, Map(0L -> 1L), 0)) { (acc, a) =>
    val prefixSum = acc.prefixSum + a
    val ans = acc.ans + acc.counter.get(prefixSum - k).getOrElse(0L)
    val counter = acc.counter.updatedWith(prefixSum) {
      case None    => Some(1L)
      case Some(c) => Some(c + 1L)
    }
    State(prefixSum, counter, ans)
  }

  println(acc.ans)

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
