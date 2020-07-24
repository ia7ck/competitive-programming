import strutils, sequtils, algorithm, future, math

const inf = int.high div 3

proc chmin(a: var int, b: int) =
  if a > b: a = b

type SegmentTree = object
  n: int
  dat: seq[int]

proc initSegmentTree(sz: int): SegmentTree =
  result.n = nextPowerOftwo(sz)
  result.dat = newSeqWith(result.n * 2 - 1, inf)

proc update(this: var SegmentTree, i: int, x: int) =
  var k = i + this.n - 1
  this.dat[k] = x
  while k > 0:
    k = (k - 1) div 2
    this.dat[k] = min(this.dat[k * 2 + 1], this.dat[k * 2 + 2])

proc rmin(this: SegmentTree, ql, qr, i, il, ir: int): int =
  if qr <= il or ir <= ql:
    return inf
  elif ql <= il and ir <= qr:
    return this.dat[i]
  else:
    let m = (il + ir) div 2
    result = min(
      this.rmin(ql, qr, i * 2 + 1, il, m), this.rmin(ql, qr, i * 2 + 2, m, ir))

proc rmin(this: SegmentTree, ql, qr: int): int =
  return rmin(this, ql, qr, 0, 0, this.n)

proc get(this: SegmentTree, i: int): int =
  return this.dat[i + this.n - 1]

proc main() =
  var n, q: int
  (n, q) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt).mapIt(it - 1)
  var
    pos = newSeq[int](n)
    seg = initSegmentTree(n)
  for i in 0..<n:
    pos[a[i]] = i
    seg.update(i, a[i])
  for qq in 0..<q:
    var t, i, j: int
    (t, i, j) = stdin.readLine.strip.split.map(parseInt)
    i -= 1
    j -= 1
    if t == 1:
      let
        ei = seg.get(i)
        ej = seg.get(j)
      seg.update(i, ej)
      seg.update(j, ei)
      pos[ei] = j
      pos[ej] = i
    else:
      echo pos[seg.rmin(i, j + 1)] + 1
main()
