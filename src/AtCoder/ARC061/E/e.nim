type HeapQueue*[T] = distinct seq[T]

proc newHeapQueue*[T](): HeapQueue[T] {.inline.} = HeapQueue[T](newSeq[T]())
proc newHeapQueue*[T](h: var HeapQueue[T]) {.inline.} = h = HeapQueue[T](
  newSeq[T]())

proc len*[T](h: HeapQueue[T]): int {.inline.} = seq[T](h).len
proc `[]`*[T](h: HeapQueue[T], i: int): T {.inline.} = seq[T](h)[i]
proc `[]=`[T](h: var HeapQueue[T], i: int,
  v: T) {.inline.} = seq[T](h)[i] = v
proc add[T](h: var HeapQueue[T], v: T) {.inline.} = seq[T](h).add(v)

proc heapCmp[T](x, y: T): bool {.inline.} =
  return (x < y)

# 'heap' is a heap at all indices >= startpos, except possibly for pos.  pos
# is the index of a leaf with a possibly out-of-order value.  Restore the
# heap invariant.
proc siftdown[T](heap: var HeapQueue[T], startpos, p: int) =
  var pos = p
  var newitem = heap[pos]
  # Follow the path to the root, moving parents down until finding a place
# newitem fits.
  while pos > startpos:
    let parentpos = (pos - 1) shr 1
    let parent = heap[parentpos]
    if heapCmp(newitem, parent):
      heap[pos] = parent
      pos = parentpos
    else:
      break
  heap[pos] = newitem

proc siftup[T](heap: var HeapQueue[T], p: int) =
  let endpos = len(heap)
  var pos = p
  let startpos = pos
  let newitem = heap[pos]
  # Bubble up the smaller child until hitting a leaf.
  var childpos = 2*pos + 1    # leftmost child position
  while childpos < endpos:
    # Set childpos to index of smaller child.
    let rightpos = childpos + 1
    if rightpos < endpos and not heapCmp(heap[childpos], heap[rightpos]):
      childpos = rightpos
    # Move the smaller child up.
    heap[pos] = heap[childpos]
    pos = childpos
    childpos = 2*pos + 1
  # The leaf at pos is empty now.  Put newitem there, and bubble it up
# to its final resting place (by sifting its parents down).
  heap[pos] = newitem
  siftdown(heap, startpos, pos)

proc push*[T](heap: var HeapQueue[T], item: T) =
  ## Push item onto heap, maintaining the heap invariant.
  (seq[T](heap)).add(item)
  siftdown(heap, 0, len(heap)-1)

proc pop*[T](heap: var HeapQueue[T]): T =
  ## Pop the smallest item off the heap, maintaining the heap invariant.
  let lastelt = seq[T](heap).pop()
  if heap.len > 0:
    result = heap[0]
    heap[0] = lastelt
    siftup(heap, 0)
  else:
    result = lastelt


type
  Edge = tuple[to: int, cost: int64]
  P = tuple[v: int, d: int64]
proc `<`(a, b: P): bool =
  return a.d < b.d

import strutils, sequtils, algorithm

proc dijkstra(g: seq[seq[Edge]], s: int): seq[int64] =
  let
    n = g.len
    inf = int64.high div 2
  var
    d = newSeqWith(n, inf)
    q = newHeapQueue[P]()
  d[s] = 0
  q.add((s, 0.int64))
  while q.len > 0:
    let cur = q.pop
    for e in g[cur.v]:
      if d[cur.v] + e.cost < d[e.to]:
        d[e.to] = d[cur.v] + e.cost
        q.add((e.to, d[e.to]))
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
