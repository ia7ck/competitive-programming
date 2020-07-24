import strutils, sequtils, math, future, algorithm

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

proc rmax[T](this: SegmentTree[T], ql, qr, i, il, ir: int): T =
  if ql <= il and ir <= qr:
    return this.dat[i]
  elif qr <= il or ir <= ql:
    return this.e
  else:
    let m = (il + ir) div 2
    return this.multiply(
      rmax(this, ql, qr, i * 2 + 1, il, m),
      rmax(this, ql, qr, i * 2 + 2, m, ir))

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[a, b: int]
  var jobs = newSeq[P]()
  for i in 0..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    jobs.add((a, b))
  jobs.sort((p, q) => cmp(q.b, p.b))
  var
    ans = 0
    st = initSegmentTree[int](m, -1, proc(a, b: int): int = max(a, b))
  for i in 0..<m:
    st.update(i, i)
  for j in jobs:
    let r = m - j.a + 1
    if r > 0:
      let i = st.rmax(0, r, 0, 0, st.n)
      if i != -1:
        ans += j.b
        st.update(i, -1)
  echo ans
main()
