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

# k = 0 溶かしてない, k = 1 溶かした
proc index(v, k: int): int =
  return v * 2 + k

type UnionFind = object
  n: int
  par: seq[int]
  card: seq[int]

proc initUnionFind(sz: int): UnionFind =
  var
    par = (0..<sz).mapIt(it)
    card = newSeqWith(sz, 1)
  result = UnionFind(n: sz, par: par, card: card)

proc find(this: var UnionFind, i: int): int =
  if i != this.par[i]:
    this.par[i] = find(this, this.par[i])
  return this.par[i]

proc unite(this: var UnionFind, i, j: int) =
  var
    ri = this.find(i)
    rj = this.find(j)
  if ri == rj:
    return
  else:
    if this.card[ri] < this.card[rj]:
      swap(ri, rj)
    this.par[rj] = ri
    this.card[ri] += this.card[rj]

proc root(this: var UnionFind, i: int): int =
  return this.find(i)

proc main() =
  var h, w, x: int
  (h, w, x) = stdin.readLine.strip.split.map(parseInt)
  var sy, sx, gy, gx: int
  (sy, sx) = stdin.readLine.strip.split.map(parseInt)
  (gy, gx) = stdin.readLine.strip.split.map(parseInt)
  let a = (0..<h).mapIt(stdin.readLine.strip.split.map(parseInt))
  let c = stdin.readLine.strip.split.map(parseInt)

  let
    dy = @[0, 0, -1, 1]
    dx = @[1, -1, 0, 0]
  var uf = initUnionFind(h * w)
  for i in 0..<h:
    for j in 0..<w:
      for k in 0..<4:
        let
          ni = i + dy[k]
          nj = j + dx[k]
        if ni >= 0 and nj >= 0 and ni < h and nj < w:
          if a[i][j] == a[ni][nj]:
            uf.unite(i * w + j, ni * w + nj)
  var g = newSeqWith(h * w * 2, newSeq[Edge]())
  for i in 0..<h:
    for j in 0..<w:
      let
        rt = uf.root(i * w + j)
        cost = if a[i][j] == 0: 0 else: c[a[i][j] - 1]
      g[index(rt, 0)].add(Edge(to: index(rt, 1), cost: cost)) # 溶かす
      for k in 0..<4:
        let
          ni = i + dy[k]
          nj = j + dx[k]
        if ni >= 0 and nj >= 0 and ni < h and nj < w:
          let
            u = uf.root(i * w + j)
            v = uf.root(ni * w + nj)
          if u == v:
            g[index(u, 1)].add(Edge(to: index(v, 1), cost: 0)) # 溶けてるとこに移動
            g[index(v, 1)].add(Edge(to: index(u, 1), cost: 0))
          else:
            g[index(u, 1)].add(Edge(to: index(v, 0), cost: 0)) # 溶けてないとこに移動
            g[index(v, 1)].add(Edge(to: index(u, 0), cost: 0))
  let
    st = uf.root((sy - 1) * w + (sx - 1))
    gl = uf.root((gy - 1) * w + (gx - 1))
    d = dijkstra(g, index(st, 1))
  echo d[index(gl, 1)]
main()
