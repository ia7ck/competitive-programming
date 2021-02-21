import strutils, sequtils, algorithm, future

proc fa(i, p: int, g: seq[seq[int]], dp: var seq[int]) =
  dp[i] = 0
  for j in g[i]:
    if j != p:
      fa(j, i, g, dp)
      dp[i] = max(dp[i], dp[j] + 1)

proc fb(i, p, d: int, g: seq[seq[int]], dp: seq[int], dist: var seq[int]) =
  dist[i] = d
  var lst = newSeq[int]()
  for j in g[i]:
    if j != p:
      dist[i] = max(dist[i], dp[j] + 1)
      lst.add(dp[j])
  if lst.len > 0:
    lst.sort(system.cmp)
    let
      mx = lst[lst.len - 1]
      mx_num = lst.filter(it => it == mx).len
    var second_mx = -1 # genius!
    for val in lst:
      if val < mx:
        second_mx = max(second_mx, val)
    for j in g[i]:
      if j != p:
        var nd = d + 1
        if dp[j] < mx or mx_num >= 2:
          nd = max(nd, mx + 2)
        else:
          nd = max(nd, second_mx + 2)
        fb(j, i, nd, g, dp, dist)
          
proc main() =
  let n1 = stdin.readLine.parseInt
  var g1 = newSeqWith(n1, newSeq[int]())
  for _ in 1..<n1:
    let pq = stdin.readLine.split.map(parseInt).map(it => it - 1)
    g1[pq[0]].add(pq[1])
    g1[pq[1]].add(pq[0])
  let n2 = stdin.readLine.parseInt
  var g2 = newSeqWith(n2, newSeq[int]())
  for _ in 1..<n2:
    let pq = stdin.readLine.split.map(parseInt).map(it => it - 1)
    g2[pq[0]].add(pq[1])
    g2[pq[1]].add(pq[0])
  var
    dp1 = newSeq[int](n1)
    dp2 = newSeq[int](n2)
  fa(0, -1, g1, dp1)
  fa(0, -1, g2, dp2)
  var
    dist1 = newSeq[int](n1)
    dist2 = newSeq[int](n2)
  fb(0, -1, 0, g1, dp1, dist1)
  fb(0, -1, 0, g2, dp2, dist2)
  dist2.sort(system.cmp)
  var acc = newSeq[int64](n2 + 1)
  for i in 0..<n2:
    acc[i + 1] = acc[i] + dist2[i]
  let diam_mx: int64 = max(dist1.max, dist2.max)
  var ans: int64 = 0
  for d in dist1:
    let k = dist2.lowerBound(int(diam_mx - d - 1))
    ans += diam_mx * k + (acc[n2] - acc[k]) + (int64)(d + 1) * (n2 - k)
  echo ans
main()
