import strutils, algorithm, sequtils, math, sugar

type SegmentTree[T] = object
  n: int
  dat: seq[T]
  e: T
  multiply: proc(a, b: T): T

proc initSegmentTree[T](n: int, e: T, f: proc(a, b: T): T): SegmentTree[T] =
  let nn = nextPowerOfTwo(n)
  var dat = newSeqWith(nn * 2 - 1, e)
  return SegmentTree[T](n: nn, dat: dat, e: e, multiply: f)

proc get[T](this: SegmentTree[T], i: int): T =
  return this.dat[i + this.n - 1]

proc update[T](this: var SegmentTree[T], i: int, x: T) =
  var k = i + this.n - 1
  this.dat[k] = x
  while k > 0:
    k = (k - 1) div 2
    this.dat[k] = this.multiply(this.dat[k * 2 + 1], this.dat[k * 2 + 2])

proc query[T](this: SegmentTree[T], ql, qr, i, il, ir: int): T =
  if ql <= il and ir <= qr:
    return this.dat[i]
  if qr <= il or ir <= ql:
    return this.e
  let m = (il + ir) div 2
  return this.multiply(
    this.query(ql, qr, i * 2 + 1, il, m),
    this.query(ql, qr, i * 2 + 2, m, ir))

proc query[T](this: SegmentTree[T], ql, qr: int): T =
  return this.query(ql, qr, 0, 0, this.n)

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc uniq[T](a: seq[T]): seq[T] =
  let sortedA = a.sorted(cmp)
  var res = newSeq[T]()
  for it in sortedA:
    if res.len == 0:
      res.add(it)
    elif res[^1] != it:
      res.add(it)
  return res

proc main() =
  let n = read().parseInt
  type P = tuple[a, b, c: int]
  var cakes = newSeq[P]()
  for i in 0..<n:
    let a, b, c = read().parseInt
    cakes.add((a, b, c))

  var bs = newSeq[int]()
  for p in cakes:
    bs.add(p.b)
  bs.sort(cmp)
  bs = bs.uniq
  proc map(b: int): int =
    return bs.lowerBound(b)
  let m = bs.len
  var seg = initSegmentTree[int64](
    m,
    0.int64,
    proc(x, y: int64): int64 = max(x, y))
  cakes.sort((p, q: P) => (if p.a == q.a: cmp(p.b, q.b) else: cmp(q.a, p.a)))
  for p in cakes:
    let i = map(p.b)
    let h = seg.query(i + 1, m)
    seg.update(i, max(seg.get(i), h + p.c))
  echo seg.query(0, m)
main()
