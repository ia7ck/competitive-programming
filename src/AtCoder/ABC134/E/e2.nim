import strutils, sequtils, math, algorithm, future

type SegmentTree[T] = object
  n: int
  dat: seq[T]
  e: T
  multiply: proc(a, b: T): T

proc initSegmentTree[T](n: int, e: T, f: proc(a, b: T): T): SegmentTree[T] =
  let nn = nextPowerOfTwo(n)
  var dat = newSeqWith(nn * 2 - 1, e)
  return SegmentTree[T](n: nn, dat: dat, e: e, multiply: f)

proc update[T](this: var SegmentTree[T], i: int, x: T) =
  var k = i + this.n - 1
  this.dat[k] = x
  while k > 0:
    k = (k - 1) div 2
    this.dat[k] = this.multiply(this.dat[k * 2 + 1], this.dat[k * 2 + 2])

proc query[T](this: SegmentTree[T], ql, qr, i, il, ir: int): T =
  if ql <= il and ir <= qr:
    return this.dat[i]
  elif qr <= il or ir <= ql:
    return this.e
  else:
    let m = (il + ir) div 2
    return this.multiply(
      query(this, ql, qr, i * 2 + 1, il, m),
      query(this, ql, qr, i * 2 + 2, m, ir))

proc main()=
  let
    n = stdin.readLine.strip.parseInt
    a = newSeqWith(n, stdin.readLine.strip.parseInt)
  var b = (0..<n).mapIt((v: a[n - it - 1], i: it))
  b.sort((p, q) => (if p.v == q.v: cmp(p.i, q.i) else: cmp(p.v, q.v)))
  var t = initSegmentTree[int](n, 0, proc(a, b: int): int = max(a, b))
  for it in b:
    let mx = t.query(0, it.i, 0, 0, t.n)
    t.update(it.i, mx + 1)
  echo t.query(0, n, 0, 0, t.n)
main()
