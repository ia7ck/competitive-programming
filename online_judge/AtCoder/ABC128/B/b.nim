import strutils, sequtils, algorithm, future

proc main() =
  let n = stdin.readLine.strip.parseInt
  type P = tuple[s: string, p: int, i: int]
  var items = newSeq[P]()
  for i in 0..<n:
    let args = stdin.readLine.strip.split
    items.add((args[0], args[1].parseInt, i + 1))
  items.sort((x, y) => (if x.s == y.s: -cmp(x.p, y.p) else: cmp(x.s, y.s)))
  echo items.mapIt($it.i).join("\n")
main()
