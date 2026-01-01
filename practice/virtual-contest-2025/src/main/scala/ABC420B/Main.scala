package ABC420B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val s = (0 until n).map(_ => scala.io.StdIn.readLine())

  val scores = (0 until m).foldLeft(Array.fill(n)(0)) { (acc, j) =>
    val (zero, one) = (0 until n).partition(i => s(i)(j) == '0')
    val minority = if zero.length <= one.length then zero else one
    for (i <- minority) {
      acc(i) += 1
    }
    acc
  }
  val maxScore = scores.max
  val ans = (0 until n).filter(i => scores(i) == maxScore)

  println(ans.map(_ + 1).mkString(" "))
