package ABC390A

@main def main() =
  val a = scala.io.StdIn.readLine().split(' ').map(_.toInt)

  val swaps = (1 until a.length).map(i => swap(a, i - 1, i))
  val ans = swaps.exists(a => a.sameElements(a.sorted))

  if ans then {
    println("Yes")
  } else {
    println("No")
  }

def swap(array: Array[Int], i: Int, j: Int): Array[Int] =
  val clone = array.clone()
  val tmp = clone(i)
  clone(i) = clone(j)
  clone(j) = tmp
  clone
