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
    query(this, ql, qr, i * 2 + 1, il, m),
    query(this, ql, qr, i * 2 + 2, m, ir))

proc query[T](this: SegmentTree[T], ql, qr: int): T =
  return query(this, ql, qr, 0, 0, this.n)

proc main() =
  let n = stdin.readLine.strip.parseInt
  var s = stdin.readLine.strip.mapIt(it.ord - 'a'.ord)
  type T = int
  var seg = initSegmentTree[T](
    n,
    0,
    proc(x, y: T): T = x or y)
  for i in 0..<n:
    seg.update(i, 1 shl s[i])
  let q = stdin.readLine.strip.parseInt
  for qq in 0..<q:
    var ln = stdin.readLine.strip.split
    if ln[0][0] == '1':
      let
        i = ln[1].parseInt - 1
        c = ln[2][0].ord - 'a'.ord
      let b = seg.get(i)
      seg.update(i, (b xor (1 shl s[i])) xor (1 shl c))
      s[i] = c
    else:
      let
        le = ln[1].parseInt
        ri = ln[2].parseInt
      let b = seg.query(le - 1, ri)
      var ans = 0
      for i in 0..<26:
        if ((b shr i) and 1) == 1:
          ans += 1
      echo ans
main()
