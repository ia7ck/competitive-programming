import strutils, sequtils, algorithm, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m, v, p = read().parseInt
  var a = newSeqWith(n, read().parseInt)

  a.sort(cmp)
  a.reverse
  var
    ok = 0
    ng = n
  proc enough(k: int): bool =
    if k < p:
      return true
    if a[k] + m < a[p - 1]:
      return false
    let
      u = v - 1 - (p - 1) - (n - k - 1)
      b = a.mapIt(a[k] + m - it)
    return b[(p - 1)..<k].sum >= u * m

  while ng - ok > 1:
    let k = (ng + ok) div 2
    if enough(k):
      ok = k
    else:
      ng = k
  echo ok + 1
main()
