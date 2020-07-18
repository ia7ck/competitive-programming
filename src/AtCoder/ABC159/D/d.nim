import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var freq = newSeq[int64](n + 1)
  for it in a:
    freq[it] += 1
  var tot: int64 = 0
  for x in 1..n:
    if freq[x] >= 2:
      tot += freq[x] * (freq[x] - 1) div 2
  for i in 0..<n:
    if freq[a[i]] >= 2:
      echo tot - (freq[a[i]] - 1)
    else:
      echo tot

main()
