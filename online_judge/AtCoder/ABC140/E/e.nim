import strutils, sequtils, math

type SegmentTree = object
  n: int
  dat: seq[int]

proc initSegmentTree(sz: int): SegmentTree =
  let n = nextPowerOfTwo(sz)
  result.n = n
  result.dat = newSeq[int](n * 2 - 1)

proc add(this: var SegmentTree, i, x: int) =
  var k = i + this.n - 1
  this.dat[k] += x
  while k > 0:
    k = (k - 1) div 2
    this.dat[k] = this.dat[k * 2 + 1] + this.dat[k * 2 + 2]

proc rsum(this: SegmentTree, ql, qr, i, il, ir: int): int =
  if qr <= il or ir <= ql:
    result = 0
  elif ql <= il and ir <= qr:
    result = this.dat[i]
  else:
    let m = (il + ir) div 2
    result = this.rsum(ql, qr, i * 2 + 1, il, m) +
      this.rsum(ql, qr, i * 2 + 2, m, ir)

proc rsum(this: SegmentTree, ql, qr: int): int =
  return this.rsum(ql, qr, 0, 0, this.n)

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    p = stdin.readLine.strip.split.map(parseInt).mapIt(it - 1)
  var
    pos = newSeq[int](n)
    seg = initSegmentTree(n + 4)
  for i in 0..<n:
    pos[p[i]] = i
  seg.add(0, 1)
  seg.add(1, 1)
  seg.add(n + 2, 1)
  seg.add(n + 3, 1)
  var ans: int64 = 0
  for v in countdown(n - 1, 0):
    let j = pos[v] + 2
    var
      ok = 0
      ng = j
    while ng - ok > 1:
      let
        m = (ng + ok) div 2
        cnt = seg.rsum(m, j)
      if cnt >= 2:
        ok = m
      else:
        ng = m
    let lt1 = ok
    (ok, ng) = (0, j)
    while ng - ok > 1:
      let
        m = (ng + ok) div 2
        cnt = seg.rsum(m, j)
      if cnt >= 1:
        ok = m
      else:
        ng = m
    let lt2 = ok
    #
    (ng, ok) = (j, n + 4)
    while ok - ng > 1:
      let
        m = (ng + ok) div 2
        cnt = seg.rsum(j, m)
      if cnt >= 2:
        ok = m
      else:
        ng = m
    let rt2 = ok
    (ng, ok) = (j, n + 4)
    while ok - ng > 1:
      let
        m = (ng + ok) div 2
        cnt = seg.rsum(j, m)
      if cnt >= 1:
        ok = m
      else:
        ng = m
    let rt1 = ok
    #
    var
      lcnt = lt2 - lt1
      rcnt = rt1 - j - 1
    if lt2 == 1:
      lcnt = 0
    ans += (v + 1) * lcnt * rcnt
    (lcnt, rcnt) = (j - lt2, rt2 - rt1)
    if rt1 == n + 3:
      rcnt = 0
    ans += (v + 1) * lcnt * rcnt
    seg.add(j, 1)
  echo ans

main()
