package ABC421C

@main def main() =
  val n = scala.io.StdIn.readInt()
  val s = scala.io.StdIn.readLine()

  val aPosS = (0 until n * 2).filter(i => s(i) == 'A')
  val ans = List(0 until n * 2 by 2, 1 until n * 2 by 2)
    .map(aPosT => aPosS.zip(aPosT).map((p, q) => (p - q).abs.toLong).sum)
    .min

  println(ans)
