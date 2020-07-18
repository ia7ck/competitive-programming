import strutils, sequtils, future, algorithm
proc main() =
  var n, t: int
  (n, t) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[a, b: int]
  var ab = newSeq[P](n)
  for it in ab.mitems:
    (it.a, it.b) = stdin.readLine.strip.split.map(parseInt)
  ab.sort((p, q) => cmp(p.a, q.a))
  const m = 10000
  var dp = newSeq[int](m)
  for it in ab:
    for s in countdown(t - 1, 0):
      if s + it.a < m:
        dp[s + it.a] = max(dp[s + it.a], dp[s] + it.b)
  echo dp.max
main()
