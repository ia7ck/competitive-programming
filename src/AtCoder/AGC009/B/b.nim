import math

type
  Queue* {.deprecated.} [T] = object ## A queue.
    data: seq[T]
    rd, wr, count, mask: int

proc initQueue*[T](initialSize: int = 4): Queue[T] =
  ## Create a new queue.
  ## Optionally, the initial capacity can be reserved via `initialSize` as a
  ## performance optimization. The length of a newly created queue will still
  ## be 0.
  ##
  ## `initialSize` needs to be a power of two. If you need to accept runtime
  ## values for this you could use the ``nextPowerOfTwo`` proc from the
  ## `math <math.html>`_ module.
  assert isPowerOfTwo(initialSize)
  result.mask = initialSize-1
  newSeq(result.data, initialSize)

proc len*[T](q: Queue[T]): int {.inline.}=
  ## Return the number of elements of `q`.
  result = q.count

template emptyCheck(q) =
  # Bounds check for the regular queue access.
  when compileOption("boundChecks"):
    if unlikely(q.count < 1):
      raise newException(IndexError, "Empty queue.")

template xBoundsCheck(q, i) =
  # Bounds check for the array like accesses.
  when compileOption("boundChecks"):  # d:release should disable this.
    if unlikely(i >= q.count):  # x < q.low is taken care by the Natural parameter
      raise newException(IndexError,
                         "Out of bounds: " & $i & " > " & $(q.count - 1))

proc front*[T](q: Queue[T]): T {.inline.}=
  ## Return the oldest element of `q`. Equivalent to `q.pop()` but does not
  ## remove it from the queue.
  emptyCheck(q)
  result = q.data[q.rd]

proc back*[T](q: Queue[T]): T {.inline.} =
  ## Return the newest element of `q` but does not remove it from the queue.
  emptyCheck(q)
  result = q.data[q.wr - 1 and q.mask]

proc `[]`*[T](q: Queue[T], i: Natural) : T {.inline.} =
  ## Access the i-th element of `q` by order of insertion.
  ## q[0] is the oldest (the next one q.pop() will extract),
  ## q[^1] is the newest (last one added to the queue).
  xBoundsCheck(q, i)
  return q.data[q.rd + i and q.mask]

proc `[]`*[T](q: var Queue[T], i: Natural): var T {.inline.} =
  ## Access the i-th element of `q` and returns a mutable
  ## reference to it.
  xBoundsCheck(q, i)
  return q.data[q.rd + i and q.mask]

proc `[]=`* [T] (q: var Queue[T], i: Natural, val : T) {.inline.} =
  ## Change the i-th element of `q`.
  xBoundsCheck(q, i)
  q.data[q.rd + i and q.mask] = val

iterator items*[T](q: Queue[T]): T =
  ## Yield every element of `q`.
  var i = q.rd
  for c in 0 ..< q.count:
    yield q.data[i]
    i = (i + 1) and q.mask

iterator mitems*[T](q: var Queue[T]): var T =
  ## Yield every element of `q`.
  var i = q.rd
  for c in 0 ..< q.count:
    yield q.data[i]
    i = (i + 1) and q.mask

iterator pairs*[T](q: Queue[T]): tuple[key: int, val: T] =
  ## Yield every (position, value) of `q`.
  var i = q.rd
  for c in 0 ..< q.count:
    yield (c, q.data[i])
    i = (i + 1) and q.mask

proc contains*[T](q: Queue[T], item: T): bool {.inline.} =
  ## Return true if `item` is in `q` or false if not found. Usually used
  ## via the ``in`` operator. It is the equivalent of ``q.find(item) >= 0``.
  ##
  ## .. code-block:: Nim
  ##   if x in q:
  ##     assert q.contains x
  for e in q:
    if e == item: return true
  return false

proc add*[T](q: var Queue[T], item: T) =
  ## Add an `item` to the end of the queue `q`.
  var cap = q.mask+1
  if unlikely(q.count >= cap):
    var n = newSeq[T](cap*2)
    for i, x in pairs(q):  # don't use copyMem because the GC and because it's slower.
      shallowCopy(n[i], x)
    shallowCopy(q.data, n)
    q.mask = cap*2 - 1
    q.wr = q.count
    q.rd = 0
  inc q.count
  q.data[q.wr] = item
  q.wr = (q.wr + 1) and q.mask

template default[T](t: typedesc[T]): T =
  var v: T
  v

proc pop*[T](q: var Queue[T]): T {.inline, discardable.} =
  ## Remove and returns the first (oldest) element of the queue `q`.
  emptyCheck(q)
  dec q.count
  result = q.data[q.rd]
  q.data[q.rd] = default(type(result))
  q.rd = (q.rd + 1) and q.mask

proc enqueue*[T](q: var Queue[T], item: T) =
  ## Alias for the ``add`` operation.
  q.add(item)

proc dequeue*[T](q: var Queue[T]): T =
  ## Alias for the ``pop`` operation.
  q.pop()

proc `$`*[T](q: Queue[T]): string =
  ## Turn a queue into its string representation.
  result = "["
  for x in items(q):  # Don't remove the items here for reasons that don't fit in this margin.
    if result.len > 1: result.add(", ")
    result.add($x)
  result.add("]")

# https://github.com/nim-lang/Nim/blob/master/lib/pure/collections/queues.nim

import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = @[-1] & newSeqWith(n - 1, read().parseInt - 1)
  var deg = newSeq[int](n)
  for i in 1..<n:
    deg[a[i]] += 1
  var dp = newSeq[int](n)
  fill(dp, -1)
  var
    b = newSeqWith(n, newSeq[int]())
    q = initQueue[int]()
  for i in 0..<n:
    if deg[i] == 0:
      q.enqueue(i)
  while q.len > 0:
    let i = q.dequeue
    b[i].sort(cmp)
    b[i].reverse
    dp[i] = 0
    for j in 0..<b[i].len:
      dp[i] = max(dp[i], b[i][j] + j + 1)
    # echo i, " ", b, " ", dp
    if a[i] >= 0:
      b[a[i]].add(dp[i])
      deg[a[i]] -= 1
      if deg[a[i]] == 0:
        q.enqueue(a[i])
  echo dp[0]
main()
