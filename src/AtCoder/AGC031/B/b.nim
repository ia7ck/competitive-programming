import strutils, sequtils, algorithm

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    c = (0..<n).mapIt(stdin.readLine.strip.parseInt)
    mo = 1000000000 + 7
  var
    last = newSeqWith(c.max + 1, n + 1)
    dp = newSeq[int64](n + 1)
  dp[0] = 1
  for i in 1..n:
    dp[i] = dp[i - 1]
    let color = c[i - 1]
    if last[color] + 1 < i:
      dp[i] = (dp[i] + dp[last[color]]) mod mo
    last[color] = i
  echo dp[n]
main()
