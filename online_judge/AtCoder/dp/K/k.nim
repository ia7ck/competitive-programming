import strutils, sequtils, intsets

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseInt)
  var g = newSeq[int](k + 1)
  g[0] = 0
  for x in 1..k:
    var s = initIntSet()
    for it in a:
      if x - it >= 0:
        s.incl(g[x - it])
    var v = 0
    while s.contains(v):
      v += 1
    g[x] = v
  if g[k] > 0:
    echo "First"
  else:
    echo "Second"

main()
