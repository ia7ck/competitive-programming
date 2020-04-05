import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, newSeqWith(n, read().parseBiggestInt))

  var pts = newSeq[int64](1 shl n)
  for s in 0..<(1 shl n):
    for i in 0..<n:
      for j in 0..<i:
        if ((s shr i) and 1) == 1 and ((s shr j) and 1) == 1:
          pts[s] += a[i][j]
  var dp = newSeq[int64](1 shl n)
  for s in 0..<(1 shl n):
    var t = s
    while t >= 0:
      t = t and s
      dp[s] = max(dp[s], dp[s xor t] + pts[t])
      t -= 1
  echo dp[(1 shl n) - 1]
main()
