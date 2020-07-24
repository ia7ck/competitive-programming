import strutils, sequtils, algorithm, future, math

type SegmentTree[T] = object
  n: int
  dat: seq[T]
  e: T
  multiply: proc(a, b: T): T

proc initSegmentTree[T](n: int, e: T, f: proc(a, b: T): T): SegmentTree[T] =
  let nn = nextPowerOfTwo(n)
  var dat = newSeqWith(nn * 2 - 1, e)
  return SegmentTree[T](n: nn, dat: dat, e: e, multiply: f)

proc get[T](this: SegmentTree[T], i: int): T =
  return this.dat[i + this.n - 1]

proc update[T](this: var SegmentTree[T], i: int, x: T) =
  var k = i + this.n - 1
  this.dat[k] = x
  while k > 0:
    k = (k - 1) div 2
    this.dat[k] = this.multiply(this.dat[k * 2 + 1], this.dat[k * 2 + 2])

proc query[T](this: SegmentTree[T], ql, qr, i, il, ir: int): T =
  if ql <= il and ir <= qr:
    return this.dat[i]
  if qr <= il or ir <= ql:
    return this.e
  let m = (il + ir) div 2
  return this.multiply(
    query(this, ql, qr, i * 2 + 1, il, m),
    query(this, ql, qr, i * 2 + 2, m, ir))

proc query[T](this: SegmentTree[T], ql, qr: int): T =
  return query(this, ql, qr, 0, 0, this.n)

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var robots = newSeqWith(n, (x: read().parseInt, d: read().parseInt))
  
  robots.sort((a, b) => cmp(a.x, b.x))
  # var robotIdx = newSeq[int](n) # ロボット i を動かしたときに消える最大のロボットの番号
  type T = int
  var seg = initSegmentTree[T](
    n,
    -1,
    proc(x, y: T): T = max(x, y))
  var dp = newSeq[int64](n + 1)
  dp[n] = 1
  const mo: int64 = 998244353
  for i in countdown(n - 1, 0):
    seg.update(i, i)
    let
      r = robots[i]
      j = robots.lowerBound(
        (x: r.x + r.d, d: -1),
        (a, b) => cmp(a.x, b.x))
      k = seg.query(i, j)
    seg.update(i, k)
    dp[i] = (dp[i + 1] + dp[k + 1]) mod mo
  echo dp[0]
main()
