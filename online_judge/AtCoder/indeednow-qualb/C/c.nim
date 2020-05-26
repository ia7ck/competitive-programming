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
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  var q = initHeapQueue[int](
    proc(x, y: int): bool = x < y)
  var
    ans = newSeq[int]()
    seen = newSeq[bool](n)
  q.push(0)
  seen[0] = true
  while ans.len < n:
    let i = q.pop
    ans.add(i)
    for j in g[i]:
      if not seen[j]:
        q.push(j)
        seen[j] = true
  echo ans.mapIt($(it + 1)).join(" ")
main()
