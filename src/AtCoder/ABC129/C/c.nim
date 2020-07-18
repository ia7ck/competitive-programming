import strutils, sequtils

const mo: int64 = 1000000000 + 7
proc add(a: var int64, b: int64) =
  a = (a + b) mod mo

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  let a = (0..<m).mapIt(stdin.readLine.strip.parseInt)

  var ng = newSeq[bool](n + 1)
  for it in a:
    ng[it] = true
  var dp = newSeq[int64](n + 1)
  dp[0] = 1
  for i in 0..<n:
    if not ng[i + 1]:
      add(dp[i + 1], dp[i])
    if i + 2 <= n and (not ng[i + 2]):
      add(dp[i + 2], dp[i])
  echo dp[n]
main()
