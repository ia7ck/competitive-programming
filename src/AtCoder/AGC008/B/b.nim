import strutils, sequtils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
  var
    ri: int64 = a[k..^1].filterIt(it > 0).sum
    le: int64 = 0
    cul = newSeq[int64](n + 1)
  for i in 1..n:
    cul[i] = cul[i - 1] + a[i - 1]
  var ans: int64 = 0
  for i in 0..(n - k):
    let
      s = max(0, cul[i + k] - cul[i])
      t = le + ri
    # echo i, " ", s, " ", le, " ", ri
    ans = max(ans, s + t)
    if a[i] > 0:
      le += a[i]
    if i + k < n and a[i + k] > 0:
      ri -= a[i + k]
  echo ans
main()
