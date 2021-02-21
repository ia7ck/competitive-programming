import strutils, sequtils

type Edge = object
  to, cost: int

proc dfs(i, color: int, c: var seq[int], g: seq[seq[Edge]]) =
  c[i] = color
  for e in g[i]:
    if c[e.to] == -1:
      dfs(e.to, (color + e.cost) mod 2, c, g)

proc main() =
  let n = stdin.readLine.strip.parseInt
  var g = newSeqWith(n, newSeq[Edge]())
  for i in 0..<(n - 1):
    var u, v, w: int
    (u, v, w) = stdin.readLine.strip.split.map(parseInt)
    u -= 1
    v -= 1
    g[u].add(Edge(to: v, cost: w mod 2))
    g[v].add(Edge(to: u, cost: w mod 2))
  var c = newSeqWith(n, -1)
  dfs(0, 0, c, g)
  echo c.mapIt($it).join("\n")
main()
