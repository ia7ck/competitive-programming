import strutils, sequtils, math, tables

proc main() =
  let n = stdin.readLine.strip.parseInt
  type P = tuple[a: int, b: int]
  var tab = initTable[P, bool]()
  for i in 0..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    tab[(min(a, b), max(a, b))] = true
  echo tab.len

main()
