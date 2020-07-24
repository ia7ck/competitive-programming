import strutils, sequtils, algorithm

# m 以下が k ペア以上あるか
proc f(pos, neg: seq[int64], z, k, m: int64): bool =
  var cnt: int64 = 0
  if m >= 0:
    for i in 0..<pos.len:
      let a = pos.lowerBound(m div pos[i] + 1) - i - 1
      cnt += max(0, a)
    for i in 0..<neg.len:
      let a = neg.lowerBound(m div neg[i] + 1) - i - 1
      cnt += max(0, a)
    cnt += pos.len * neg.len
    cnt += z * pos.len
    cnt += z * neg.len
    cnt += z * (z - 1) div 2
  elif m < 0:
    for it in pos:
      let y = (-m + it - 1) div it
      cnt += neg.len - neg.lowerBound(y)
  return cnt >= k

proc main() =
  var nn, k: int64
  (nn, k) = stdin.readLine.strip.split.map(parseBiggestInt)
  let n = nn.int
  let a = stdin.readLine.strip.split.map(parseBiggestInt)

  var
    neg = a.filterIt(it < 0).mapIt(-it)
    pos = a.filterIt(it > 0)
    zero = a.filterIt(it == 0).len
  neg.sort(cmp)
  pos.sort(cmp)
  var
    ok = 1_000_000_000_000_000_000
    ng = -(ok + 1)
  while ok - ng > 1:
    let m = (ok + ng) div 2
    if f(pos, neg, zero, k, m):
      ok = m
    else:
      ng = m
  echo ok
main()
