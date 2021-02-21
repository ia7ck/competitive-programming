import strutils, sequtils, algorithm, future

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[v: int, i: int]
  var ai = newSeq[P]()
  for i in 0..<n:
    ai.add((a[i], i + 1))
  ai.sort((x, y) => cmp(x.v, y.v))
  echo ai.mapIt(it.i).mapIt($it).join(" ")
main()
