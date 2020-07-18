import strutils, sequtils, math

type E = tuple[to, i: int]
proc dfs(i, p, c: int, g: seq[seq[E]], color: var seq[int]) =
  var d = 1
  for e in g[i]:
    if e.to == p: continue
    if d == c:
      d += 1
    color[e.i] = d
    dfs(e.to, i, d, g, color)
    d += 1


proc main() =
  let n = stdin.readLine.strip.parseInt
  var g = newSeqWith(n, newSeq[E]())
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    g[a - 1].add((b - 1, i - 1))
    g[b - 1].add((a - 1, i - 1))
  var deg = newSeq[int](n)
  for i in 0..<n:
    for e in g[i]:
      deg[e.to] += 1
  let
    k = deg.max
    r = deg.find(k)
  var color = newSeq[int](n - 1)
  dfs(r, -1, -1, g, color)
  echo k
  echo color.mapIt($it).join("\n")
main()
