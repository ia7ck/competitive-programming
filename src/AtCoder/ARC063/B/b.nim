import strutils, sequtils, intsets

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, t = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var
    mn = a[0]
    mx = 0
  for it in a[1..^1]:
    mx = max(mx, it - mn)
    mn = min(mn, it)
  var
    s = initIntSet()
    ans = 0
  for it in a:
    if s.contains(it - mx):
      ans += 1
    s.incl(it)
  echo ans
main()
