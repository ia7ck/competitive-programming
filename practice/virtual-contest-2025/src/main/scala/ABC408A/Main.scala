package ABC408A

import scala.util.boundary, scala.util.boundary.break

@main def main(): Unit =
  boundary:
    val Array(n, s) = scala.io.StdIn.readLine().split(' ').map(_.toInt)
    val t = scala.io.StdIn.readLine().split(' ').map(_.toInt)

    var last = 0
    for (t <- t) {
      if last + s < t then {
        println("No")
        break()
      }
      last = t
    }

    println("Yes")
