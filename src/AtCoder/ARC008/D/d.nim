import strutils, sequtils, algorithm

type Node = object
  a, b: float64
proc merge(ln, rn: Node): Node =
  return Node(a: rn.a*ln.a, b: rn.a*ln.b+rn.b)
type SegmentTree = object
  n: int
  dat: seq[Node]
proc initSegmentTree(sz: int): SegmentTree =
  var n = 1
  while n<sz: n*=2
  var dat = newSeqWith(n*2-1, Node(a: 1, b: 0))
  return SegmentTree(n: n, dat: dat)
proc update(this: var SegmentTree, i: int, a, b: float64) =
  var k = i+this.n-1
  this.dat[k].a = a
  this.dat[k].b = b
  while k>0:
    k = (k-1) div 2
    this.dat[k] = merge(this.dat[k*2+1], this.dat[k*2+2])
proc query(this: SegmentTree, ql, qr, i, il, ir: int): Node =
  if ir<=ql or qr<=il:
    return Node(a: 1, b: 0)
  elif ql<=il and ir<=qr:
    return this.dat[i]
  else:
    let m = (il+ir) div 2
    return merge(
      this.query(ql, qr, i*2+1, il, m), this.query(ql, qr, i*2+2, m, ir))
      
proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    pab_list = (0..<m).mapIt(stdin.readLine.split.map(parseFloat))
  var ps = newSeq[int]()
  for pab in pab_list:
    ps.add(pab[0].int)
  ps.sort(system.cmp)
  ps = deduplicate(ps)
  var
    tree = initSegmentTree(ps.len)
    mn = 1.0
    mx = 1.0
  for pab in pab_list:
    let j = ps.lowerBound(pab[0].int)
    tree.update(j, pab[1], pab[2])
    let res = tree.query(0, ps.len, 0, 0, tree.n)
    mn = min(mn, res.a+res.b)
    mx = max(mx, res.a+res.b)
  echo mn
  echo mx
main()
