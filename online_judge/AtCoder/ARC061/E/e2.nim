type HeapQueue[T] = object
  dat: seq[T]
  cmp: proc(x, y: T): bool

proc `[]`[T](h: HeapQueue[T], i: int): T = h.dat[i]
proc `[]=`[T](h: var HeapQueue[T], i: int, x: T) = h.dat[i] = x
proc len[T](h: HeapQueue[T]): int = h.dat.len

proc initHeapQueue[T](cmp: proc(x, y: T): bool): HeapQueue[T] =
  return HeapQueue[T](dat: newSeq[T](), cmp: cmp)

proc swap[T](h: var HeapQueue[T], i, j: int) =
  swap(h.dat[i], h.dat[j])

proc push[T](h: var HeapQueue[T], x: T) =
  h.dat.add(x)
  var i = h.len - 1
  while i > 0:
    var p = (i - 1) div 2
    if h.cmp(h[i], h[p]):
      h.swap(i, p)
      i = p
    else:
      break

proc pop[T](h: var HeapQueue[T]): T =
  assert(h.len >= 1)
  h.swap(0, h.len - 1)
  let x = h.dat.pop
  var i = 0
  while i < h.len:
    let
      left = i * 2 + 1
      right = i * 2 + 2
    if left >= h.len:
      break
    var ch = left
    if right < h.len and h.cmp(h[right], h[ch]):
      ch = right
    if h.cmp(h[ch], h[i]):
      h.swap(ch, i)
    i = ch
  return x

type
  Edge = tuple[to: int, cost: int64]
  P = tuple[v: int, d: int64]

import strutils, sequtils, algorithm

proc dijkstra(g: seq[seq[Edge]], s: int): seq[int64] =
  let
    n = g.len
    inf = int64.high div 2
  var
    d = newSeqWith(n, inf)
    q = initHeapQueue[P](proc(x, y: P): bool = x.d < y.d)
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
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt
  var
    g = newSeqWith(n + m  * 2 + 1, newSeq[Edge]())
    companies = newSeqWith(n, newSeq[int]())
    edges = newSeq[(int, int, int)]()
  for i in 0..<m:
    let a, b, c = read().parseInt
    companies[a - 1].add(c)
    companies[b - 1].add(c)
    edges.add((a - 1, b - 1, c))
  for i in 0..<n:
    companies[i].sort(cmp)
  var acc = newSeq[int](n)
  for i in 1..<n:
    acc[i] = acc[i - 1] + companies[i - 1].len + 1
  proc map(v, c: int): int =
    if c == 0:
      return acc[v]
    let j = companies[v].lowerBound(c)
    return acc[v] + j + 1
  for it in edges:
    let (a, b, c) = it
    g[map(a, c)].add((map(b, c), 0.int64))
    g[map(b, c)].add((map(a, c), 0.int64))
    g[map(a, c)].add((map(a, 0), 0.int64))
    g[map(b, c)].add((map(b, 0), 0.int64))
    g[map(a, 0)].add((map(a, c), 1.int64))
    g[map(b, 0)].add((map(b, c), 1.int64))
  let
    s = map(0, 0)
    t = map(n - 1, 0)
    inf = int64.high div 2
    d = dijkstra(g, s)
  var ans = d[t]
  if ans == inf:
    ans = -1
  echo ans
main()
