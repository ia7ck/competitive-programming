package ABC432A

@main def main() =
  val abc = scala.io.StdIn.readLine()

  val ans = abc.permutations.max

  println(ans)
