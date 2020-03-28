import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    k, n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  var ans = a[^1] - a[0]
  for i in 0..<(n - 1):
    ans = min(ans, k - (a[i + 1] - a[i]))
  echo ans

main()
