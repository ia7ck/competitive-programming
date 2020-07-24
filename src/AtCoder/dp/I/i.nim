import strutils, sequtils
proc printf(formatstr: cstring) {.importc: "printf", varargs,
  header: "<stdio.h>".}
proc main() =
  let
    n = stdin.readLine.parseInt
    p = stdin.readLine.split.map(parseFloat)
  var dp = newSeqWith(n+1, newSeq[float64](n+1))
  dp[0][0] = 1.0
  for i in 0..<n:
    for j in 0..i:
      dp[i+1][j+1] += dp[i][j]*p[i]
      dp[i+1][j] += dp[i][j]*(1-p[i])
  var ans = 0.0
  for j in (n div 2)+1..n:
    ans+=dp[n][j]
  printf("%.18f\n", ans)
main()
