import strutils, sequtils, algorithm

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

proc dijkstra(g: seq[seq[Edge]], s: int, inf: int64): seq[int64] =
  let n = g.len
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
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    fst = read()
    lst = read()
  var
    n = read().parseInt
    words = newSeqWith(n, read())

  if fst.len != lst.len:
    echo -1
    return
  if fst == lst:
    echo 0
    echo fst
    echo lst
    return
  if fst.len != words[0].len:
    echo -1
    return
  words.add(fst)
  words.add(lst)
  words = words.deduplicate
  n = words.len
  let
    start = words.find(fst)
    goal = words.find(lst)
  var g = newSeqWith(n, newSeq[Edge]())
  for i in 0..<n:
    for j in 0..<i:
      var diff = 0
      for k in 0..<words[i].len:
        if words[i][k] != words[j][k]:
          diff += 1
      if diff == 1:
        g[i].add((j, 1.int64))
        g[j].add((i, 1.int64))
  const inf = int64.high div 2
  let d = dijkstra(g, start, inf)
  if d[goal] >= inf:
    echo -1
    return
  echo d[goal] - 1
  var
    cur = goal
    ans = newSeq[string]()
  while cur != start:
    ans.add(words[cur])
    for e in g[cur]:
      if d[e.to] + 1 == d[cur]:
        cur = e.to
        break
  ans.add(fst)
  ans.reverse
  echo ans.mapIt($it).join("\n")
main()
