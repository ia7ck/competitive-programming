package D

@main def main =
  val fs = FastScanner(java.lang.System.in)
  val n = fs.nextInt()
  val k = fs.nextInt()
  val m = fs.nextLong()
  val a = Array.fill(n) { fs.nextLong() }

  var l = 0
  var sum = 0L
  var ans = 0L
  for (r <- 0 until n) {
    sum += a(r)
    while (l <= r && r - l + 1 >= k && sum >= m) {
      sum -= a(l)
      l += 1
    }
    ans += l.toLong
  }

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
