import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, m = read().parseInt
    h = newSeqWith(n, read().parseInt)
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<m:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  var ans = 0
  for i in 0..<n:
    var ok = true
    for j in g[i]:
      if h[j] >= h[i]:
        ok = false
    if ok:
      ans += 1
  echo ans
main()
