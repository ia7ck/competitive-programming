import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
  if a.filterIt(it == 1).len == 0:
    echo -1
    return
  var dp = newSeq[int](n)
  for i in 0..<n:
    if a[i] == 1:
      dp[i] = 1
    if i > 0:
      dp[i] = dp[i - 1]
      if dp[i - 1] + 1 == a[i]:
        dp[i] = dp[i - 1] + 1
  echo n - dp[^1]
main()
