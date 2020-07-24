import strutils, sequtils, math
proc main() =
  var a, b: int64
  (a, b) = stdin.readLine.strip.split.map(parseBiggestInt)
  echo lcm(a, b)
main()
