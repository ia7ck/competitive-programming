import strutils, sequtils, algorithm, future, math

const inf = 1000000000 + 7
type SegmentTree = object
  n: int
  dat: seq[int]
  laz: seq[int]
proc initSegmentTree(len: int): SegmentTree =
  let n = nextPowerOfTwo(len) # 頂点数
  var
    dat = newSeqWith(n * 2 - 1, inf)
    laz = newSeqWith(n * 2 - 1, -1)
  return SegmentTree(n: n, dat: dat, laz: laz)
proc set(this: var SegmentTree, i, x: int) =
  this.dat[i] = x
  this.laz[i] = x
proc push(this: var SegmentTree, i: int) =
  if this.laz[i] >= 0:
    assert(i * 2 + 1 < this.n * 2 - 1 and i * 2 + 2 < this.n * 2 - 1)
    this.set(i * 2 + 1, this.laz[i])
    this.set(i * 2 + 2, this.laz[i])
    this.laz[i] = -1
proc update(this: var SegmentTree, ql, qr, x, i, il, ir: int) =
  if qr <= il or ir <= ql:
    return
  elif ql <= il and ir <= qr:
    this.set(i, x)
    return
  else:
    let m = (il + ir) div 2
    this.push(i)
    this.update(ql, qr, x, i * 2 + 1, il, m)
    this.update(ql, qr, x, i * 2 + 2, m, ir)
    this.dat[i] = min(this.dat[i * 2 + 1], this.dat[i * 2 + 2])
proc get_min(this: var SegmentTree, ql, qr, i, il, ir: int): int =
  if qr <= il or ir <= ql:
    return inf
  elif ql <= il and ir <= qr:
    return this.dat[i]
  else:
    let m = (il + ir) div 2
    this.push(i)
    return min(
      this.get_min(ql, qr, i * 2 + 1, il, m),
      this.get_min(ql, qr, i * 2 + 2, m, ir))

proc main() =
  var n, q: int
  (n, q) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[s, t, x: int]
  var items = newSeq[P]()
  for i in 0..<n:
    var s, t, x: int
    (s, t, x) = stdin.readLine.strip.split.map(parseInt)
    items.add((s, t, x))
  let d = (0..<q).mapIt(stdin.readLine.strip.parseInt)

  items.sort((a, b) => -cmp(a.x, b.x))
  var seg = initSegmentTree(q)
  for it in items:
    let
      left = d.lowerBound(it.s - it.x)
      right = d.lowerBound(it.t - it.x)
    if left < right:
      seg.update(left, right, it.x, 0, 0, seg.n)
  for i in 0..<q:
    var ans = seg.get_min(i, i + 1, 0, 0, seg.n)
    if ans == inf:
      ans = -1
    echo ans
main()
