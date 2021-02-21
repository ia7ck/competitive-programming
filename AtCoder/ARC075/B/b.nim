import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, a, b = read().parseInt
    h = newSeqWith(n, read().parseBiggestInt)

  var
    ok = 1_000_000_000
    ng = 0
  while ok - ng > 1:
    let
      m = (ok + ng) div 2
      r = h.mapIt(max(0, it - m * b))
    var s: int64 = 0
    for it in r:
      s += (it + (a - b) - 1) div (a - b)
    if s <= m:
      ok = m
    else:
      ng = m

  echo ok

main()
