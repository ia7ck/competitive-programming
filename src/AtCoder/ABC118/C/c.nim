import strutils, sequtils, math
proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseInt)
  var g = gcd(a[0], a[1])
  for i in 2..<n:
    g = gcd(g, a[i])
  echo g
main()
