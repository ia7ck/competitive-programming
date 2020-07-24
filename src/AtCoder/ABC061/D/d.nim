import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m = read().parseInt
  type E = tuple[to: int, cost: int64]
  var g = newSeqWith(n, newSeq[E]())
  for i in 0..<m:
    let a, b, c = read().parseInt
    g[a - 1].add((b - 1, -1 * c.int64))
  const inf = int64.high div 2
  var dp = newSeqWith(n * 2 + 1, newSeqWith(n, inf))
  dp[0][0] = 0
  for i in 0..<(n * 2):
    for u in 0..<n:
      if dp[i][u] == inf:
        continue
      dp[i + 1][u] = min(dp[i + 1][u], dp[i][u])
      for e in g[u]:
        dp[i + 1][e.to] = min(dp[i + 1][e.to], dp[i][u] + e.cost)
  var upd = dp[n - 1][n - 1] > dp[n * 2][n - 1]
  if upd:
    echo "inf"
    return
  echo (-1 * dp[n - 1][n - 1])
main()
