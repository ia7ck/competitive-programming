import strutils, sequtils, heapqueue

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

type
  Edge = tuple[to: int, cost: int64]
  P = tuple[v: int, d: int64]

proc `<`(x, y: P): bool = x.d < y.d

proc dijkstra(g: seq[seq[Edge]], s: int, inf: int64): seq[int64] =
  let n = g.len
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

proc main() =
  let n, m = read().parseInt
  var edges = newSeq[(int, int)]()
  for i in 0..<m:
    let a, b = read().parseInt
    edges.add((a - 1, b - 1))

  var g = newSeqWith(n, newSeq[Edge]())
  for e in edges:
    let (a, b) = e
    g[a].add((b, 1.int64))
    g[b].add((a, 1.int64))
  const inf = int64.high div 2
  let d = dijkstra(g, 0, inf)
  if d.anyIt(it == inf):
    echo "No"
    return
  echo "Yes"
  var to = newSeq[int](n)
  for e in edges:
    let (a, b) = e
    if d[a] + 1 == d[b]:
      to[b] = a
    if d[b] + 1 == d[a]:
      to[a] = b
  echo to[1..^1].mapIt(it + 1).join("\n")
main()
