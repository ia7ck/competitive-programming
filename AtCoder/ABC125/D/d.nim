import strutils, sequtils, math

proc chmax(a: var int64, b: int64) =
  if a < b: a = b
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseBiggestInt)
    inf = int64.high div 3
  var dp = newSeqWith(n + 1, newSeqWith(2, -inf))
  dp[0][0] = 0
  dp[0][1] = 0
  for i in 0..<n:
    for j in 0..<2:
      chmax(dp[i + 1][0], dp[i].max + a[i])
      if i > 0:
        chmax(dp[i + 1][1], dp[i][0] - a[i - 1] - a[i - 1] - a[i])
        chmax(dp[i + 1][1], dp[i][1] + a[i - 1] + a[i - 1] - a[i])
  echo dp[n].max

main()
