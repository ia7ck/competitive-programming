import strutils, sequtils

proc chmax(a: var int, b: int) =
  if a < b: a = b

proc main() =
  let
    s = stdin.readLine.strip
    t = stdin.readLine.strip
    n = s.len
    m = t.len
  # s[0, i) と t[0, j) の最長共通なんとか
  var dp = newSeqWith(n + 1, newSeq[int](m + 1))
  for i in 1..n:
    for j in 1..m:
      chmax(dp[i][j], dp[i][j - 1])
      chmax(dp[i][j], dp[i - 1][j])
      if s[i - 1] == t[j - 1]:
        chmax(dp[i][j], dp[i - 1][j - 1] + 1)
  echo dp[n][m] + 1

main()
