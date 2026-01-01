package ABC389B

@main def main() =
  val x = scala.io.StdIn.readLong()

  val Some(ans) = LazyList.from(1).find(n => factorial(n) == x): @unchecked

  println(ans)

def factorial(n: Long): Long =
  if n == 0 then {
    1
  } else {
    n * factorial(n - 1)
  }
