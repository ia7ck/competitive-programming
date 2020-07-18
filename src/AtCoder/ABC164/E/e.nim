import strutils, heapqueue, sequtils

type
  E = tuple[to: int, cost: int64]
  P = tuple[v: int, d: int64]
proc `<`(a, b: P): bool =
  return a.d < b.d

proc dijkstra(g: seq[seq[E]], s: int): seq[int64] =
  let
    n = g.len
    inf = int64.high div 2
  var
    d = newSeqWith(n, inf)
    q = initHeapQueue[P]()
  d[s] = 0
  q.push((s, 0.int64))
  while q.len > 0:
    let cur = q.pop
    for e in g[cur.v]:
      if d[cur.v] + e.cost < d[e.to]:
        d[e.to] = d[cur.v] + e.cost
        q.push((e.to, d[e.to]))
  return d

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let N, M, S = read().parseInt
  type Edge = tuple[u, v, a, b: int]
  var edges = newSeq[Edge]()
  for i in 0..<M:
    let u, v, a, b = read().parseInt
    edges.add((u - 1, v - 1, a, b))
  type vertex = tuple[c, d: int]
  var vers = newSeq[vertex]()
  for i in 0..<N:
    let c, d = read().parseInt
    vers.add((c, d))

  var sum = 0
  for e in edges:
    sum += e.a
  proc index(u, p: int): int =
    return u * (sum + 1) + p
  let n = N * (sum + 1) + (sum + 1)
  var g = newSeqWith(n, newSeq[E]())
  for e in edges:
    for p in e.a..sum:
      g[index(e.u, p)].add((index(e.v, p - e.a), e.b.int64))
      g[index(e.v, p)].add((index(e.u, p - e.a), e.b.int64))
  for u in 0..<N:
    var (c, d) = vers[u]
    # c = min(c, sum)
    for p in 0..sum:
      g[index(u, p)].add((index(u, min(sum, p + c)), d.int64))
  let dist = dijkstra(g, index(0, min(S, sum)))
  for t in 1..<N:
    var ans = int64.high
    for p in 0..sum:
      ans = min(ans, dist[index(t, p)])
    echo ans
main()
