import strutils, sequtils, math

proc main() =
  var a, b, c, d: int64
  (a, b, c, d) = stdin.readLine.strip.split.map(parseBiggestInt)

  let
    g = gcd(c, d)
    p = (c div g) * d

  var res = (b div c) - ((a - 1) div c)
  res += (b div d) - ((a - 1) div d)
  res -= (b div p) - ((a - 1) div p)

  echo (b - a + 1 - res)


main()
