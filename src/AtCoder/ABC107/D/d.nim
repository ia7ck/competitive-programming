import strutils, sequtils

type SegmentTree = object
  n: int
  dat: seq[int]
proc init(sz: int): SegmentTree =
  var n = 1
  while n<sz: n*=2
  var dat = newSeqWith(n*2-1, 0)
  result = SegmentTree(n: n, dat: dat)
proc add(this: var SegmentTree, i, x: int) =
  var k = i+this.n-1
  this.dat[k]+=x
  while k>0:
    k = (k-1) div 2
    this.dat[k] = this.dat[k*2+1]+this.dat[k*2+2]
proc rsum(this: SegmentTree, ql, qr, i, il, ir: int): int =
  if qr<=il or ir<=ql:
    result = 0
  elif ql<=il and ir<=qr:
    result = this.dat[i]
  else:
    let m = (il+ir) div 2
    result = this.rsum(ql, qr, i*2+1, il, m)+this.rsum(ql, qr, i*2+2, m, ir)

let
  n = stdin.readLine.parseInt
  a = stdin.readLine.split.map(parseInt)

proc count(m: int): int64 =
  var b = newSeq[int](n+1)
  b[0] = 0
  for i in 0..<n:
    b[i+1] = if a[i]<=m: -1 else: 1
    b[i+1]+=b[i]
  let mn = b.min
  for i in 0..n:
    b[i]-=mn
  let sz = b.max+1
  var tree = init(sz)
  for val in b:
    result+=tree.rsum(val+1, sz, 0, 0, tree.n)
    tree.add(val, 1)

proc main() =
  var
    lb = 0
    ub = 1_000_000_000+1
  while ub-lb>1:
    let m = (ub+lb) div 2
    if count(m)>=n*(n+1) div 4 + 1:
      ub = m
    else:
      lb = m
  echo ub
main()
