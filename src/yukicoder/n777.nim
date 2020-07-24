import strutils, sequtils, algorithm, tables

type SegmentTree = object
  n: int
  dat: seq[int64]
proc init(sz: int): SegmentTree =
  var n = 1
  while n<sz: n*=2
  var dat = newSeqWith(n*2-1, 0'i64)
  result = SegmentTree(n: n, dat: dat)
proc update(this: var SegmentTree, i: int, x: int64) =
  var k = i+this.n-1
  if this.dat[k]<x:
    this.dat[k] = x
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
  let n = stdin.readLine.parseInt
  var abcs = (0..<n).mapIt(stdin.readLine.split.map(parseInt))
  abcs.sort(proc(x, y: seq[int]): int = cmp(x[1], y[1]))
  var map = newTable[int64, int]()
  for abc in abcs:
    if not map.hasKey(abc[1]):
      map[abc[1]] = map.len
  abcs.sort do (x, y: seq[int])->int:
    result = cmp(y[0], x[0])
    if result==0:
      result = cmp(x[1], y[1])
  let m = map.len
  var
    tree = init(m)
    ans = 0'i64
  for i, abc in abcs:
    let s = tree.rmax(map[abc[1]]+1, m, 0, 0, tree.n) + abc[2]
    ans = max(ans, s)
    tree.update(map[abc[1]], s)
  echo ans
main()
