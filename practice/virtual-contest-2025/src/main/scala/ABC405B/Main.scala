package ABC405B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.inits.find(b => !(1 to m).forall(x => b.contains(x))) match {
    case Some(b) => a.length - b.length
    case None    => n
  }

  println(ans)
