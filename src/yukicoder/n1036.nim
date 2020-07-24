import strutils, sequtils, math

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
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  var seg = initSegmentTree[int64](
    n,
    0,
    proc(x, y: int64): int64 = gcd(x, y))
  for i in 0..<n:
    seg.update(i, a[i])
  var
    ans: int64 = 0
    le = 0
  for ri in 1..n:
    while le < ri and seg.query(le, ri) == 1:
      ans += n - ri + 1
      le += 1
  echo ans
main()
