package ABC416C

@main def main() =
  val Array(n, k, x) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val s = Array.fill(n) { scala.io.StdIn.readLine() }

  val allStringsSorted =
    gen(k, 0 until n).map(a => a.map(s).mkString("")).sorted
  val ans = allStringsSorted(x - 1)

  println(ans)

def gen(length: Int, valueRange: Range): Seq[List[Int]] =
  if (length == 0) then {
    Seq(Nil)
  } else {
    for {
      v <- valueRange
      l <- gen(length - 1, valueRange)
    } yield v :: l
  }
