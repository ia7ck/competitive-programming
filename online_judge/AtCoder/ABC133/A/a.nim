import strutils, sequtils, algorithm

proc main() =
  var n, a, b: int
  (n, a, b) = stdin.readLine.strip.split.map(parseInt)

  echo min(n * a, b)
main()
