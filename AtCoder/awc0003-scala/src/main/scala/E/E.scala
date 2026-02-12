package E

@main def main =
  val fs = FastScanner(java.lang.System.in)
  val n = fs.nextInt()
  val m = fs.nextInt()
  val w = Array.fill(n) { fs.nextLong() }
  val c = Array.fill(m) { fs.nextLong() }

  val w_sum = Array.fill(1 << n)(0L)
  for {
    s <- 0 until (1 << n)
    i <- 0 until n
    if (s >> i & 1) == 1
  } {
    w_sum(s) += w(i)
  }

  val initDP = Array.fill(1 << n)(false)
  initDP(0) = true
  val dp = c.foldLeft(initDP) { (dp, c) =>
    val newDP = dp.clone()

    for (s <- 0 until (1 << n)) {
      var t = s
      while (t >= 0) {
        t &= s
        newDP(s) |= dp(t) && w_sum(s - t) <= c
        t -= 1
      }
    }

    newDP
  }

  if dp.last then {
    println("Yes")
  } else {
    println("No")
  }

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
