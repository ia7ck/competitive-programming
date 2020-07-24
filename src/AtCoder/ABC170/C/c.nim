import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    x, n = read().parseInt
    a = newSeqWith(n, read().parseInt)

  let m = 123
  var ok = newSeqWith(m, true)
  for x in a:
    ok[x] = false
  var
    diff = int.high
    ans = 0
  for i in 0..<m:
    if ok[i]:
      let d = abs(i - x)
      if d < diff:
        ans = i
        diff = d
  echo ans

main()
