package ABC404A

@main def main() =
  val s = scala.io.StdIn.readLine()

  val Some(ans) = ('a' to 'z').find(c => !s.contains(c)): @unchecked

  println(ans)
