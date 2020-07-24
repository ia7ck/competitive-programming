import strutils, sequtils, algorithm

proc main() =
  var a, b, c: int
  (a, b, c) = stdin.readLine.strip.split.map(parseInt)

  echo max(0, c - (a - b))

main()
