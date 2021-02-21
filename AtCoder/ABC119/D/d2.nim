import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    A, B, Q = read().parseInt
    s = newSeqWith(A, read().parseBiggestInt)
    t = newSeqWith(B, read().parseBiggestInt)
    xs = newSeqWith(Q, read().parseBiggestInt)
  for x in xs:
    var ans = int64.high
    let
      i = s.lowerBound(x) - 1
      j = t.lowerBound(x) - 1
    # echo i, " ", j
    if i >= 0 and j >= 0:
      ans = min(ans, max(abs(x - s[i]), abs(x - t[j])))
    if i + 1 < A and j + 1 < B:
      ans = min(ans, max(abs(x - s[i + 1]), abs(x - t[j + 1])))
    if i >= 0 and j + 1 < B:
      ans = min(ans, abs(x - s[i]) * 2 + abs(x - t[j + 1]))
      ans = min(ans, abs(x - t[j + 1]) * 2 + abs(x - s[i]))
    if j >= 0 and i + 1 < A:
      ans = min(ans, abs(x - t[j]) * 2 + abs(x - s[i + 1]))
      ans = min(ans, abs(x - s[i + 1]) * 2 + abs(x - t[j]))
    echo ans
main()
