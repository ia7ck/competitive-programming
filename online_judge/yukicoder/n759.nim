import strutils, sequtils, algorithm

type P = tuple[x: int, y: int, z: int, idx: int]
type SegmentTree = object
  n: int
  dat: seq[int]
  list: seq[seq[int]]
proc init(pts: seq[P]): SegmentTree =
  var m = 1
  for pt in pts: m = max(m, pt.y+1)
  var n = 1
  while n<m: n*=2
  var list = newSeqWith(n, newSeq[int](0))
  for pt in pts: list[pt.y].add(pt.z)
  var dat = newSeqWith(n*2-1, -1)
  for i in 0..<m: dat[i+n-1] = if list[i].len>0: list[i].max else: -1
  var i = n-2
  while i>=0:
    dat[i] = max(dat[i*2+1], dat[i*2+2])
    i-=1
  result = SegmentTree(n: n, dat: dat, list: list)
proc del(this: var SegmentTree, y: int) =
  this.list[y].delete(0, 0)
  var i = y+this.n-1
  this.dat[i] = if this.list[y].len>0: this.list[y].max else: -1
  while i>0:
    i = (i-1) div 2
    this.dat[i] = max(this.dat[i*2+1], this.dat[i*2+2])
proc maxz(this: SegmentTree, ql, qr, i, il, ir: int): int =
  if qr<=il or ir<=ql:
    result = -1
  elif ql<=il and ir<=qr:
    result = this.dat[i]
  else:
    let m = (il+ir) div 2
    result = max(
      this.maxz(ql, qr, i*2+1, il, m),
      this.maxz(ql, qr, i*2+2, m, ir)
    )

let n = stdin.readLine.parseInt
var
  pts = newSeq[P](n)
  ymax = 0
for i in 0..<n:
  (pts[i].x, pts[i].y, pts[i].z) = stdin.readLine.split.map(parseInt)
  pts[i].idx = i
  ymax = max(ymax, pts[i].y)
pts.sort do (p, q: P) -> int:
  result = cmp(p.x, q.x)
  if result==0:
    result = cmp(p.y, q.y)
    if result==0:
      result = cmp(p.z, q.z)
var
  tree = init(pts)
  ans = newSeq[int]()
for pt in pts:
  tree.del(pt.y)
  let z = tree.maxz(pt.y, ymax+1, 0, 0, tree.n)
  if pt.z>z:
    ans.add(pt.idx)
ans.sort(system.cmp)
for a in ans: echo a+1
