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

proc `<`[T](a, b: T): bool {.inline.} =
  return a.dist < b.dist

proc main() =
  let
    nm = stdin.readLine.strip.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    abci = (0..<m).mapIt(stdin.readLine.strip.split.map(parseInt))

  type Edge = object
    to: int
    cost: int64
  var g = newSeqWith(n, newSeq[Edge]())
  for abc in abci:
    g[abc[0] - 1].add(Edge(to: abc[1] - 1, cost: abc[2]))
    g[abc[1] - 1].add(Edge(to: abc[0] - 1, cost: abc[2]))
  type T = object
    node: int
    dist: int64
    used: int
  var
    q = newHeapQueue[T]()
    dist = newSeqWith(n, newSeqWith(2, 100000000000000000))
  dist[0][0] = 0
  dist[0][1] = 0
  q.add(T(node: 0, dist: 0, used: 0))
  while q.len > 0:
    let cur = q.pop
    for e in g[cur.node]:
      if cur.dist + e.cost < dist[e.to][cur.used]:
        dist[e.to][cur.used] = cur.dist + e.cost
        q.add(T(node: e.to, dist: dist[e.to][cur.used], used: cur.used))
      if (cur.used == 0) and (cur.dist < dist[e.to][1]):
        dist[e.to][1] = cur.dist
        q.add(T(node: e.to, dist: dist[e.to][1], used: 1))
  for i in 0..<n:
    echo dist[i].sum

main()
