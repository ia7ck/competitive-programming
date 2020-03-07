import strutils, sequtils, math, future

type SegmentTree[T] = object
  n: int
  dat: seq[T]
  e: T
  multiply: (T, T)->T

proc initSegmentTree[T](n: int, e: T, f: (T, T)->T): SegmentTree[T] =
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
    query(this, ql, qr, i * 2 + 1, il, m),
    query(this, ql, qr, i * 2 + 2, m, ir))

proc query[T](this: SegmentTree[T], ql, qr: int): T =
  return query(this, ql, qr, 0, 0, this.n)

# thx: https://qiita.com/CUteNeuron/items/f0900f17d8d1c652855e
let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt
  var s = read().mapIt(it.ord - 'a'.ord)
  type T = int
  var seg = initSegmentTree[T](
    n,
    0,
    proc(x, y: T): T = x or y)
  for i in 0..<n:
    seg.update(i, 1 shl s[i])
  let q = read().parseInt
  for qq in 0..<q:
    let t = read().parseInt
    if t == 1:
      let
        i = read().parseInt - 1
        c = read()[0].ord - 'a'.ord
      let b = seg.get(i)
      seg.update(i, (b xor (1 shl s[i])) xor (1 shl c))
      s[i] = c
    else:
      let (le, ri) = (read().parseInt, read().parseInt)
      let b = seg.query(le - 1, ri)
      echo b.int32.countBits32
main()
