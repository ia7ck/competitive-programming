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

# https://github.com/nim-lang/Nim/blob/master/lib/pure/collections/heapqueue.nim
# ---------------------------------------------------------------------------------------------- #
# ---------------------------------------------------------------------------------------------- #

import strutils, sequtils, math

type Edge = object
  to: int
  cost: int64

type P = object
  v: int
  d: int64

proc `<`(a, b: P): bool =
  return a.d < b.d

proc dijkstra(g: seq[seq[Edge]], s: int): seq[int64] =
  let
    n = g.len
    inf = int64.high div 2
  var
    d = newSeqWith(n, inf)
    q = newHeapQueue[P]()
  d[s] = 0
  q.add(P(v: s, d: 0))
  while q.len > 0:
    let cur = q.pop
    for e in g[cur.v]:
      if d[cur.v] + e.cost < d[e.to]:
        d[e.to] = d[cur.v] + e.cost
        q.add(P(v: e.to, d: d[e.to]))
  return d

const K = 1000

proc index(v, k: int): int =
  return v * K + k

proc main() =
  var n, m, k: int
  (n, m, k) = stdin.readLine.strip.split.map(parseInt)
  var g = newSeqWith(n * K + k, newSeq[Edge]())
  for i in 0..<m:
    var a, b, c: int
    (a, b, c) = stdin.readLine.strip.split.map(parseInt)
    a -= 1
    b -= 1
    for j in 0..k:
      g[index(a, j)].add(Edge(to: index(b, j), cost: c))
      g[index(b, j)].add(Edge(to: index(a, j), cost: c))
  for i in 0..<n:
    var x, y: int
    (x, y) = stdin.readLine.strip.split.map(parseInt)
    if x > 0:
      for j in 0..<k:
        g[index(i, j)].add(Edge(to: index(i, min(k, j + x)), cost: y))
  let d = dijkstra(g, index(0, 0))
  var ans = d[index(n - 1, k)]
  if ans == int64.high div 2:
    ans = -1
  echo ans
main()
