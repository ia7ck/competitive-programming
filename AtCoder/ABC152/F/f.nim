import strutils, sequtils, algorithm
type E = tuple[to, idx: int]
var
  g: seq[seq[E]]
  dep: seq[int]
  par: seq[int]
proc dfs(i, p: int) =
  par[i] = p
  for e in g[i]:
    if e.to == p: continue
    dep[e.to] = dep[i] + 1
    dfs(e.to, i)
proc getEdges(i, j: int): seq[int] =
  var
    u = i
    v = j
  if dep[u] < dep[v]:
    swap(u, v)
  result = newSeq[int]()
  while dep[u] != dep[v]:
    result.add(u)
    u = par[u]
  while u != v:
    result.add(u)
    result.add(v)
    u = par[u]
    v = par[v]

proc main() =
  let n = stdin.readLine.strip.parseInt
  g = newSeqWith(n, newSeq[E]()) # seq[seq[int]] でよかったかも
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    g[a - 1].add((b - 1, i))
    g[b - 1].add((a - 1, i))
  dep = newSeq[int](n)
  par = newSeq[int](n)
  dfs(0, -1)
  let m = stdin.readLine.strip.parseInt
  var edges = newSeq[seq[int]](m)
  for i in 0..<m:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    edges[i] = getEdges(a - 1, b - 1)
  var ans: int64 = 1
  for i in 1..<n: ans = ans * 2
  for bits in 1..<(1 shl m):
    var free = newSeq[bool](n) # 0 は使わない
    fill(free, true)
    for i in 0..<m:
      if ((bits shr i) and 1) == 1:
        for j in edges[i]:
          free[j] = false
    var ret: int64 = 1
    for i in 1..<n:
      if free[i]:
        ret = ret * 2
    var cnt = 0
    for i in 0..<m:
      if ((bits shr i) and 1) == 1:
        cnt += 1
    if (cnt and 1) == 1:
      ans -= ret
    else:
      ans += ret
  echo ans
main()
