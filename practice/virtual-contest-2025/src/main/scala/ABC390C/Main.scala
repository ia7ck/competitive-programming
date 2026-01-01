package ABC390C

@main def main() =
  val Array(h, w) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
  val s = (0 until h).map(_ => scala.io.StdIn.readLine())

  val blacks = for {
    i <- 0 until h
    j <- 0 until w
    if s(i)(j) == '#'
  } yield (i, j)
  val top = blacks.map((i, _) => i).min
  val bottom = blacks.map((i, _) => i).max
  val left = blacks.map((_, j) => j).min
  val right = blacks.map((_, j) => j).max
  val rectangle = for {
    i <- top to bottom
    j <- left to right
  } yield s(i)(j)
  val ans = rectangle.forall(c => c == '#' || c == '?')

  if ans then println("Yes")
  else println("No")
