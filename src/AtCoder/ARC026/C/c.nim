import strutils, sequtils, algorithm, future, math

const inf: int64 = 1000000000000000000

proc chmin(a: var int64, b: int64) =
  if a > b: a = b

type SegmentTree = object
  n: int
  dat: seq[int64]
proc initSegmentTree(sz: int): SegmentTree =
  let n = nextPowerOftwo(sz)
  var dat = newSeqWith(n * 2 - 1, inf)
  result = SegmentTree(n: n, dat: dat)
proc update(this: var SegmentTree, i: int, x: int64) =
  var k = i + this.n-1
  chmin(this.dat[k], x)       # !!!
  while k > 0:
    k = (k - 1) div 2
    this.dat[k] = min(this.dat[k * 2 + 1], this.dat[k * 2 + 2])
proc rmin(this: SegmentTree, ql, qr, i, il, ir: int): int64 =
  if qr <= il or ir <= ql:
    return inf
  elif ql <= il and ir <= qr:
    return this.dat[i]
  else:
    let m = (il + ir) div 2
    result = min(
      this.rmin(ql, qr, i * 2 + 1, il, m), this.rmin(ql, qr, i * 2 + 2, m, ir))

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[left: int, right: int, cost: int]
  var items = newSeq[P](n)
  for i in 0..<n:
    var a, b, c: int
    (a, b, c) = stdin.readLine.strip.split.map(parseInt)
    items[i] = (a, b, c)
  items.sort((a, b) => cmp(a.right, b.right))
  var s = initSegmentTree(m + 1)
  s.update(0, 0)
  for it in items:
    let c = s.rmin(it.left, it.right, 0, 0, s.n)
    s.update(it.right, c + it.cost)
  echo s.rmin(m, m + 1, 0, 0, s.n)
main()
