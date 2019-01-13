import strutils, sequtils

proc main() =
  let
    nk = stdin.readLine.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
    a = stdin.readLine.split.map(parseInt)
    MOD = 1_000_000_000+7
  var dp = newSeqWith(n+1, newSeqWith(k+1, 0))
  dp[0][0] = 1
  for i in 0..<n:
    var s = 0
    for j in 0..k:
      s = (s+dp[i][j]) mod MOD
      if j-a[i]>0:
        s = (s-dp[i][j-a[i]-1]+MOD) mod MOD
      dp[i+1][j] = s
  echo dp[n][k]

main()
