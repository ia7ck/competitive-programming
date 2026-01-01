package ABC429B

@main def main() =
  val Array(n, m) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val ans = a.combinations(n - 1).exists(a => a.sum == m)

  if ans then println("Yes")
  else println("No")
