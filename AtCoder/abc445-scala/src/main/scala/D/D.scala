package D

import scala.annotation.tailrec

@main def main =
  val fs = FastScanner(java.lang.System.in)
  val h = fs.nextInt()
  val w = fs.nextInt()
  val n = fs.nextInt()
  val hw = Array.fill(n) {
    val h = fs.nextInt()
    val w = fs.nextInt()
    (h, w)
  }

  val (hs, ws) = hw.unzip
  val orderByH = List.range(0, n).sortBy(hs).reverse
  val orderByW = List.range(0, n).sortBy(ws).reverse

  val ans =
    solve(
      State(h, w, orderByH, orderByW, Set.empty, List.empty),
      Context(hs, ws)
    )
      .sortBy(_.i)
      .map(a => s"${a.x} ${a.y}")
      .mkString("\n")
  println(ans)

case class State(
    h: Int,
    w: Int,
    orderByH: List[Int],
    orderByW: List[Int],
    used: Set[Int],
    acc: List[Answer]
)

case class Answer(i: Int, x: Int, y: Int)

case class Context(h: Array[Int], w: Array[Int])

@tailrec
def solve(
    s: State,
    ctx: Context
): List[Answer] =
  (s.orderByH, s.orderByW) match {
    case (Nil, Nil)                           => s.acc
    case (i :: rest, _) if s.used.contains(i) =>
      solve(s.copy(orderByH = rest), ctx)
    case (i :: rest, _) if ctx.h(i) == s.h => {
      val nextW = s.w - ctx.w(i)
      solve(
        s.copy(
          w = nextW,
          orderByH = rest,
          used = s.used.incl(i),
          acc = Answer(i, 1, nextW + 1) :: s.acc
        ),
        ctx
      )
    }
    case (_, i :: rest) if s.used.contains(i) =>
      solve(s.copy(orderByW = rest), ctx)
    case (_, i :: rest) if ctx.w(i) == s.w => {
      val nextH = s.h - ctx.h(i)
      solve(
        s.copy(
          h = nextH,
          orderByW = rest,
          used = s.used.incl(i),
          acc = Answer(i, nextH + 1, 1) :: s.acc
        ),
        ctx
      )
    }
    case _ => ???
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
