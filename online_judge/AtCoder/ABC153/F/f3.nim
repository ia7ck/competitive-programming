import strutils, sequtils, algorithm, math, future

type SegmentTree[T, U] = object
  n: int
  et: T
  multiply: (T, T)->T
  dat: seq[T]
  eu: U
  composite: (U, U)->U
  apply: (U, T)->T
  laz: seq[U]

proc initSegmentTree[T, U](
    n: int, a: seq[T],
    et: T, mul: (T, T)->T,
    eu: U, com: (U, U)->U, apply: (U, T)->T): SegmentTree[T, U] =
  let nn = nextPowerOfTwo(n)
  var
    dat = newSeqWith(nn * 2 - 1, et)
    laz = newSeqWith(nn * 2 - 1, eu)
  for i in 0..<n:
    dat[i + nn - 1] = a[i]
  for i in countdown(nn - 2, 0):
    dat[i] = mul(dat[i * 2 + 1], dat[i * 2 + 2])
  return SegmentTree[T, U](
    n: nn,
    et: et, multiply: mul, dat: dat,
    eu: eu, composite: com, apply: apply, laz: laz)

proc updateNode[T, U](this: var SegmentTree[T, U], i: int, f: U) =
  this.dat[i] = this.apply(f, this.dat[i])
  this.laz[i] = this.composite(f, this.laz[i])

proc update[T, U](this: var SegmentTree[T, U], ql, qr, i, il, ir: int, f: U) =
  if ql <= il and ir <= qr:
    this.updateNode(i, f)
    return
  if qr <= il or ir <= ql:
    return
  let
    m = (il + ir) div 2
    lch = i * 2 + 1
    rch = i * 2 + 2
  this.updateNode(lch, this.laz[i])
  this.updateNode(rch, this.laz[i])
  this.laz[i] = this.eu
  this.update(ql, qr, lch, il, m, f)
  this.update(ql, qr, rch, m, ir, f)
  this.dat[i] = this.multiply(this.dat[lch], this.dat[rch])

proc update[T, U](this: var SegmentTree[T, U], ql, qr: int, f: U) =
  this.update(ql, qr, 0, 0, this.n, f)

proc query[T, U](this: SegmentTree[T, U], ql, qr, i, il, ir: int): T =
  if ql <= il and ir <= qr:
    return this.dat[i]
  if qr <= il or ir <= ql:
    return this.et
  let m = (il + ir) div 2
  return this.apply(
    this.laz[i],
    this.multiply(
      this.query(ql, qr, i * 2 + 1, il, m),
      this.query(ql, qr, i * 2 + 2, m, ir)))

proc query[T, U](this: SegmentTree[T, U], ql, qr: int): T =
  return this.query(ql, qr, 0, 0, this.n)

# thx: https://qiita.com/CUteNeuron/items/f0900f17d8d1c652855e
let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let (n, d, a) = (
    read().parseInt, read().parseBiggestInt, read().parseBiggestInt)
  type P = tuple[x, h: int64]
  var items = newSeqWith(
    n, (x: read().parseBiggestInt, h: read().parseBiggestInt))
  items.sort((p, q: P) => cmp(p.x, q.x))
  const inf = int64.high div 2
  type
    T = int64
    U = int64
  var
    ans: int64 = 0
    st = initSegmentTree[T, U](
      n, items.mapIt(it.h),
      -inf,
      proc(a, b: T): T = max(a, b),
      0,
      proc(f, g: U): U = f + g,
      proc(f: U, x: T): T = (if x == -inf: x else: f + x))
  for i in 0..<n:
    let
      (x, h) = items[i]
      mx = st.query(0, i + 1)
      j = items.lowerBound(
        (x + d * 2 + 1, 0.int64), (p, q: P) => cmp(p.x, q.x))
    if mx > 0:
      let k = (mx + a - 1) div a
      ans += k
      st.update(i, j, k * a * (-1))
  echo ans
main()
