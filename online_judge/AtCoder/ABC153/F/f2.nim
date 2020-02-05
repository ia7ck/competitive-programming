import strutils, sequtils, algorithm, math, future

type SegmentTree[T, U] = object
  n: int
  et: T
  multiply: proc(a, b: T): T
  dat: seq[T]
  eu: U
  composite: proc(f, g: U): U
  apply: proc(f: U, x: T): T
  laz: seq[U]

proc initSegmentTree[T, U](n: int, a: seq[T],
    et: T, mul: proc(a, b: T): T,
    eu: U, com: proc(f, g: U): U, apply: proc(f: U, x: T): T): SegmentTree[T, U] =
  let nn = nextPowerOfTwo(n)
  var
    dat = newSeqWith(nn * 2 - 1, et)
    laz = newSeqWith(nn * 2 - 1, eu)
  for i in 0..<n:
    dat[i + nn - 1] = a[i]
  for i in countdown(nn - 2, 0):
    dat[i] = mul(dat[i * 2 + 1], dat[i * 2 + 2])
  return SegmentTree[T, U](n: nn,
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

proc query[T, U](this: SegmentTree[T, U], ql, qr, i, il, ir: int): T =
  if ql <= il and ir <= qr:
    return this.dat[i]
  if qr <= il or ir <= ql:
    return this.et
  let m = (il + ir) div 2
  return this.apply(this.laz[i],
    this.multiply(
      this.query(ql, qr, i * 2 + 1, il, m),
      this.query(ql, qr, i * 2 + 2, m, ir)))

proc main() =
  var
    n: int
    d, a: int64
  let nda = stdin.readLine.strip.split.map(parseInt)
  (n, d, a) = (nda[0], nda[1].int64, nda[2].int64)
  type P = tuple[x, h: int64]
  var items = newSeq[P]()
  for i in 0..<n:
    var x, h: int64
    (x, h) = stdin.readLine.strip.split.map(parseBiggestInt)
    items.add((x, h))
  items.sort(proc(p, q: P): int = cmp(p.x, q.x))
  const inf = int64.high div 2
  type T = int64
  type U = int64
  var
    ans: int64 = 0
    st = initSegmentTree[T, U](n, items.mapIt(it.h),
      -inf,
      proc(a, b: T): T = max(a, b),
      0,
      proc(f, g: U): U = f + g,
      proc(f: U, x: T): T = (if x == -inf: x else: f + x))
  for i in 0..<n:
    var x, h: int64
    (x, h) = items[i]
    let
      mx = st.query(0, i + 1, 0, 0, st.n)
      j = items.lowerBound(
        (x + d * 2 + 1, 0.int64), proc(p, q: P): int = cmp(p.x, q.x))
    if mx > 0:
      let k = (mx + a - 1) div a
      ans += k
      st.update(i, j, 0, 0, st.n, k * a * (-1))
  echo ans
main()
