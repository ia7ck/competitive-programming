import strutils, sequtils, algorithm

type SegmentTree = object
  n: int
  dat: seq[int64]
proc initSegmentTree(sz: int): SegmentTree =
  var n = 1
  while n<sz: n*=2
  var dat = newSeqWith(n*2-1, 0'i64)
  result = SegmentTree(n: n, dat: dat)
proc add(this: var SegmentTree, i: int, x: int64) =
  var k = i+this.n-1
  this.dat[k]+=x
  while k>0:
    k = (k-1) div 2
    this.dat[k] = max(this.dat[k*2+1], this.dat[k*2+2])
proc rmax(this: SegmentTree, ql, qr, i, il, ir: int): int64 =
  if qr<=il or ir<=ql:
    result = 0
  elif ql<=il and ir<=qr:
    result = this.dat[i]
  else:
    let m = (il+ir) div 2
    result = max(
      this.rmax(ql, qr, i*2+1, il, m), this.rmax(ql, qr, i*2+2, m, ir))

proc main() =
  let
    n = stdin.readLine.parseInt
    h = stdin.readLine.split.map(parseInt)
    a = stdin.readLine.split.map(parseBiggestInt)
  type T = object
    h, i: int
    a: int64
  var items = newSeq[T](n)
  for i in 0..<n:
    items[i] = T(h: h[i], i: i, a: a[i])
  items.sort(proc(x, y: T): int = cmp(x.h, y.h))
  var t = initSegmentTree(n)
  for it in items:
    let mx = t.rmax(0, it.i, 0, 0, t.n)
    t.add(it.i, mx+it.a)
  echo t.rmax(0, n, 0, 0, t.n)
main()
