import strutils, sequtils, algorithm

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

proc f(g: seq[seq[int]], a: var seq[int], i: int) =
  a.add(i)
  for j in g[i]:
    f(g, a, j)
    a.add(i)

proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseInt)
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<(n-1):
    g[a[i]].add(i+1)
  var eul = newSeq[int]()
  f(g, eul, 0)
  var
    pos_first = newSeqWith(n, -1)
    pos_last = newSeqWith(n, -1)
  for i, v in eul:
    if pos_first[v]<0:
      pos_first[v] = i
  for i, v in reversed(eul):
    if pos_last[v]<0:
      pos_last[v] = eul.len-i
  var
    tree = init(eul.len)
    ans = 0'i64
  for i in 1..n:
    ans+=tree.rsum(pos_first[n-i], pos_last[n-i], 0, 0, tree.n)
    tree.add(pos_first[n-i], 1)
  echo ans
main()
