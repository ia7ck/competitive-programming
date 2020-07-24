import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, m, k = read().parseInt
    a = newSeqWith(n, read().parseInt)
    b = newSeqWith(m, read().parseInt)
  var ans = a.min + b.min
  for i in 0..<k:
    let x, y, c = read().parseInt
    ans = min(ans, a[x - 1] + b[y - 1] - c)
  echo ans
main()
