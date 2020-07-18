import strutils, sequtils, algorithm
proc main() =
  var h, n: int
  (h, n) = stdin.readLine.strip.split.map(parseInt)
  let ab = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))

  const m = 20000
  var dp = newSeq[int64](h + m + 1)
  const inf = 1000000000
  fill(dp, inf)
  dp[0] = 0
  for it in ab:
    let
      a = it[0]
      b = it[1]
    for j in 0..<(h + m + 1):
      if j >= a:
        dp[j] = min(dp[j], dp[j - a] + b)
  echo dp[h..^1].min
main()
