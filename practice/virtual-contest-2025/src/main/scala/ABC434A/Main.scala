package ABC434A

@main def main() =
  val Array(w, b) = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val Some(ans) = LazyList.from(1).find(n => w * 1000 < n * b): @unchecked

  println(ans)
