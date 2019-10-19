import strutils, sequtils, algorithm

proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)
  echo max(0, a - b * 2)
main()
