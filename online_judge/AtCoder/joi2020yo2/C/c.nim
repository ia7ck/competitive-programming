import strutils, sequtils, algorithm, math

proc main()=
  let n = stdin.readLine.strip.parseInt
  var dp = newSeq[int64](n + 1)
  fill(dp, 1)
  for i in 1..n:
    let s = ($i).mapIt(it.ord - '0'.ord).sum
    if i + s <= n:
      dp[i + s] += dp[i]
  echo dp[n]
main()
