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

import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  var q = initHeapQueue[int64](proc(x, y: int64): bool = x > y)
  for it in a:
    q.push(it)
  for i in 0..<m:
    let x = q.pop
    q.push(x div 2)
  var ans: int64 = 0
  while q.len > 0:
    ans += q.pop
  echo ans
main()
