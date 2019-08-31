import strutils, sequtils

type Edge = tuple[to: int, d: int]
var
  ans: int64 = 0
  dp: seq[int64]
  size: seq[int]

proc dfs(g: seq[seq[Edge]], i: int, p: int) =
  if dp[i] >= 0:
    return
  dp[i] = 0
  for e in g[i]:
    if e.to == p: continue
    dfs(g, e.to, i)
    size[i] += size[e.to]
    dp[i] += dp[e.to] + size[e.to] * e.d
  ans += dp[i]

proc sfd(g: seq[seq[Edge]], i: int, p: int, acc: int64) =
  ans += acc
  for e in g[i]:
    if e.to == p: continue
    sfd(g, e.to, i, acc +
      (dp[i] - dp[e.to] - size[e.to] * e.d) +
      (size[0] - size[e.to]) * e.d)

proc main() =
  let n = stdin.readLine.parseInt
  var g = newSeqWith(n, newSeq[Edge]())
  for i in 1..<n:
    var u, v, w: int
    (u, v, w) = stdin.readLine.split.map(parseInt)
    g[u - 1].add((v - 1, w))
    g[v - 1].add((u - 1, w))
  dp = newSeqWith(n, -1'i64)
  size = newSeqWith(n, 1)
  dfs(g, 0, -1)
  sfd(g, 0, -1, 0)
  echo ans
main()
