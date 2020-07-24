import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    c = newSeqWith(n, read().parseInt)

  var
    last = newSeq[int](200000 + 1)
    dp = newSeq[int](n)
  fill(last, -1)
  dp[0] = 1
  last[c[0]] = 0
  for i in 1..<n:
    dp[i] = dp[i - 1]
    let p = last[c[i]]
    if 0 <= p and p < i - 1:
      dp[i] = (dp[i] + dp[p]) mod 1000000007
    last[c[i]] = i
  echo dp[^1]

main()
