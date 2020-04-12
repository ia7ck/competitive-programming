import strutils, algorithm

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
# ------------------------------------------------------------------------------ #

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m = read().parseInt
  type P = tuple[a, b: int]
  var jobs = newSeq[P]()
  for i in 0..<n:
    let a, b = read().parseInt
    jobs.add((a, b))
  jobs.sort(cmp)
  var
    j = 0
    q = newHeapQueue[int]()
    ans = 0
  for i in countdown(m, 0):
    while j < n and i + jobs[j].a <= m:
      q.push(jobs[j].b * (-1))
      j += 1
    if q.len > 0:
      ans += q.pop * (-1)
  echo ans
main()
