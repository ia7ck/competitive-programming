import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt

  var dp = newSeq[int](1 shl n)
  const inf = int.high div 2
  fill(dp, inf)
  dp[0] = 0
  for q in 0..<m:
    let
      a, b = read().parseInt
      c = newSeqWith(b, read().parseInt)
    var y = 0
    for it in c:
      y = y or (1 shl (it - 1))
    for bits in 0..<(1 shl n):
      let to = bits or y
      dp[to] = min(dp[to], dp[bits] + a)
  var ans = dp[(1 shl n) - 1]
  if ans == inf:
    ans = -1
  echo ans
main()
