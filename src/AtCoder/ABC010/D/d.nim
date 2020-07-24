import strutils, sequtils, algorithm

type
  Edge = object
    to: int
    rev: int
    cap: int                  # Tにできない？
  Graph = seq[seq[Edge]]
proc addEdge(g: var Graph, fr, to, cap: int) =
  g[fr].add(Edge(to: to, rev: g[to].len, cap: cap))
  g[to].add(Edge(to: fr, rev: g[fr].len - 1, cap: 0))
proc dfs(g: var Graph, sink, i, f: int, seen: var seq[bool]): int =
  if i == sink:
    return f
  seen[i] = true
  for k, e in g[i]:
    if seen[e.to] or e.cap == 0:
      continue
    let tmpf = dfs(g, sink, e.to, min(f, e.cap), seen)
    if tmpf > 0:
      g[i][k].cap -= tmpf     # e -= tmpf
      g[e.to][e.rev].cap += tmpf
      return tmpf
  return 0
proc ford(g: var Graph, source, sink: int): int =
  var
    maxf = 0
    seen = newSeq[bool](g.len)
  while true:
    fill(seen, false)
    let f = dfs(g, sink, source, int.high, seen)
    if f > 0:
      maxf += f
    else:
      break
  return maxf
proc main() =
  let
    nkm = stdin.readLine.strip.split.map(parseInt)
    (n, k, m) = (nkm[0], nkm[1], nkm[2])
    pi = stdin.readLine.strip.split.map(parseInt)
    abi = (0..<m).mapIt(stdin.readLine.strip.split.map(parseInt))

  var g: Graph = newSeqWith(n + 1, newSeq[Edge]())
  for ab in abi:
    g.addEdge(ab[0], ab[1], 1)
    g.addEdge(ab[1], ab[0], 1)
  for p in pi:
    g.addEdge(n, p, 1)
  echo ford(g, n, 0)

main()
