import strutils, sequtils, math, tables

proc main() =
  let n = stdin.readLine.strip.parseInt
  for i in 0..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    var d = max(a, b) - min(a, b)
    if d == 0:
      d = -1
    echo d
main()
