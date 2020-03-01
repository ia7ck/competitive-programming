import strutils, sequtils, math, algorithm

type SegmentTree = object
  n: int
  dat: seq[seq[int]]

const M = 26
proc initSegmentTree(a: seq[int]): SegmentTree =
  let n = nextPowerOfTwo(a.len)
  var dat = newSeqWith(n * 2 - 1, newSeq[int](M))
  for i in 0..<a.len:
    var k = i + n - 1
    dat[k][a[i]] = 1
    while k > 0:
      k = (k - 1) div 2
      dat[k][a[i]] += 1
  return SegmentTree(n: n, dat: dat)

proc update(this: var SegmentTree, i, x, y: int) =
  var k = i + this.n - 1
  this.dat[k][x] -= 1
  this.dat[k][y] += 1
  while k > 0:
    k = (k - 1) div 2
    this.dat[k][x] -= 1
    this.dat[k][y] += 1

proc sum(this: SegmentTree, ql, qr, x, i, il, ir: int): int =
  if ir <= ql or qr <= il:
    return 0
  if ql <= il and ir <= qr:
    return this.dat[i][x]
  let m = (il + ir) div 2
  return this.sum(ql, qr, x, i * 2 + 1, il, m) + this.sum(ql, qr, x, i * 2 + 2, m, ir)

proc sum(this: SegmentTree, ql, qr, x: int): int =
  return this.sum(ql, qr, x, 0, 0, this.n)

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip
  var a = newSeq[int]()
  for ch in s:
    a.add(ch.ord - 'a'.ord)
  var seg = initSegmentTree(a)
  let q = stdin.readLine.strip.parseInt
  for qq in 0..<q:
    var ln = stdin.readLine.strip.split
    if ln[0][0] == '1':
      let
        i = ln[1].parseInt
        c = ln[2][0].ord - 'a'.ord
      seg.update(i - 1, a[i - 1], c)
      a[i - 1] = c
    else:
      let
        le = ln[1].parseInt
        ri = ln[2].parseInt
      var ans = 0
      for c in 0..<M:
        if seg.sum(le - 1, ri, c) > 0:
          ans += 1
      echo ans
main()
