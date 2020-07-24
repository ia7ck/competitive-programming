import strutils, sequtils, math, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    k = read().parseBiggestInt
  var
    a = newSeqWith(n, read().parseBiggestInt)
    f = newSeqWith(n, read().parseBiggestInt)

  if a.sum <= k:
    echo 0
    return
  a.sort(cmp)
  f.sort(cmp)
  f.reverse
  proc c(y: int64): bool =
    # (a[i] - x) * f[i] <= y,
    # x >= a[i] - y / f[i]
    var m: int64 = 0
    for i in 0..<n:
      m += max(0, a[i] - y div f[i])
    return m <= k
  var
    ok = 1000000 * 1000000 + 1
    ng = 0
  while ok - ng > 1:
    let md = (ok + ng) div 2
    if c(md):
      ok = md
    else:
      ng = md
  echo ok
main()
