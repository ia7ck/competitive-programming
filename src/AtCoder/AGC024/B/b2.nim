import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = newSeqWith(n, stdin.readLine.strip.parseInt)
  var dp = newSeq[int](n + 1)
  for it in a:
    dp[it] = dp[it - 1] + 1
  echo n - dp.max
main()
