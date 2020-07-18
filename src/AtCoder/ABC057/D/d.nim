import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, A, B = read().parseInt
    v = newSeqWith(n, read().parseBiggestInt)
  var dp = newSeqWith(n + 1, newSeq[int64](n + 1))
  dp[0][0] = 0
  for i in 0..<n:
    for j in 0..n:
      if j + 1 <= n:
        dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + v[i])
      dp[i + 1][j] = max(dp[i + 1][j], dp[i][j])
  var ep = newSeqWith(n + 1, newSeq[int64](n + 1))
  ep[0][0] = 1
  for i in 0..<n:
    for j in 0..n:
      if j + 1 <= n and dp[i][j] + v[i] == dp[i + 1][j + 1]:
        ep[i + 1][j + 1] += ep[i][j]
      if dp[i + 1][j] == dp[i][j]:
        ep[i + 1][j] += ep[i][j]
  var
    sum: int64 = 0
    nn: int64 = 1
  for i in A..B:
    # sum/nn < dp[n][i]/i
    # sum * i < dp[n][i] * nn
    if sum * i < dp[n][i] * nn:
      sum = dp[n][i]
      nn = i
  var ans: int64 = 0
  for i in A..B:
    # sum/nn == dp[n][i]/i
    if sum * i == dp[n][i] * nn:
      ans += ep[n][i]
  let ave = (sum.float64 / nn.float64).formatFloat
  echo ave
  echo ans
main()
