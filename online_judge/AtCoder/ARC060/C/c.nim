import strutils, sequtils

proc main() =
  var n, av: int
  (n, av) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  let s = 2500
  var dp = newSeqWith(n + 1, newSeq[int64](s + 1))
  dp[0][0] = 1
  for it in a:
    for j in countdown(n - 1, 0):
      for t in countdown(s - 1, 0):
        if it + t <= s:
          dp[j + 1][it + t] += dp[j][t]
  var ans: int64 = 0
  for j in 1..n:
    ans += dp[j][av * j]
  echo ans
main()
