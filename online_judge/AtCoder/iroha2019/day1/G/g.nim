import strutils, sequtils, math

proc chmax(a: var int64, b: int64) =
  if a < b: a = b

proc main() =
  var n, m, k: int
  (n, m, k) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseBiggestInt)

  var dp = newSeqWith(2, newSeqWith(m + 1, newSeqWith(n + 1, (-1).int64)))
  dp[0][0][0] = 0
  for i in 0..<n:
    let
      cur = i mod 2
      nxt = cur xor 1
    for j in 0..m:
      for t in max(0, i - k + 1)..n:
        if dp[cur][j][t] >= 0:
          chmax(dp[nxt][j][t], dp[cur][j][t])
          if j + 1 <= m:
            chmax(dp[nxt][j + 1][i + 1], dp[cur][j][t] + a[i])
  var mx: int64 = -1
  for t in (n - k + 1)..n:
    chmax(mx, dp[n mod 2][m][t])
  echo mx
main()
