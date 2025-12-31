package ABC426A

@main def main() =
  val Array(x, y) = scala.io.StdIn.readLine().split(' ')

  val versions = List("Ocelot", "Serval", "Lynx")
  val ans = versions.indexOf(x) >= versions.indexOf(y)

  if ans then {
    println("Yes")
  } else {
    println("No")
  }
