import strutils, sequtils, algorithm

proc dfs_01(i, p: int, g: seq[seq[int]], depth, par, dist: var seq[int]) =
  par[i] = p
  if p >= 0:
    depth[i] = depth[p] + 1
  for j in g[i]:
    if j != p:
      dfs_01(j, i, g, depth, par, dist)
      dist[i] = max(dist[i], dist[j] + 1)

proc main() =
  var n, u, v: int
  (n, u, v) = stdin.readLine.strip.split.map(parseInt)
  u -= 1
  v -= 1
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  var
    depth = newSeq[int](n)
    par = newSeq[int](n)
    dist = newSeq[int](n) # 部分木内の頂点への最長距離
  dfs_01(v, -1, g, depth, par, dist)
  let d0 = depth[u]
  var
    c = 0
    ans = 0
  # echo dist
  while u != v and c * 2 <= d0 - 1:
    ans = max(ans, d0 - 1 - c + dist[u])
    c += 1
    u = par[u]
  echo ans
main()
