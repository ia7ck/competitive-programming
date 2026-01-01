package ABC417B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val b = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = b.foldLeft(a) { (acc, b) =>
    val i = acc.indexOf(b)
    if i == -1 then {
      acc
    } else {
      acc.take(i) ++ acc.drop(i + 1)
    }
  }

  println(ans.mkString(" "))
