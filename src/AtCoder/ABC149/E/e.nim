import strutils, sequtils, algorithm
proc main() =
  var n, m: int64
  (n, m) = stdin.readLine.strip.split.map(parseBiggestInt)
  var a = stdin.readLine.strip.split.map(parseBiggestInt)

  a.sort(system.cmp)
  var
    ok = 0
    ng = 1234567
  while ng - ok > 1:
    let mid = (ok + ng) div 2
    # 和が mid 以上のペアを m 個以上取れるか
    var k: int64 = 0
    for it in a:
      let j = a.lowerBound(mid - it)
      k += n - j
    if k >= m:
      ok = mid
    else:
      ng = mid

  var acc = newSeq[int64](n + 1)
  for i in 0..<n.int:
    acc[i + 1] = acc[i] + a[i]
  var
    ans: int64 = 0
    k: int64 = 0
  for it in a:
    let j = a.lowerBound(ok - it)
    ans += acc[n.int] - acc[j] + it * (n - j)
    k += n - j
  ans -= (k - m) * ok
  echo ans

main()
