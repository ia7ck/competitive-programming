import strutils, sequtils, algorithm, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<m:
    let x, y = read().parseInt
    g[x].add(y)
    g[y].add(x)
  var
    seen = newSeqWith(n, false)
    cc = newSeq[seq[int]]()
  proc dfs(i: int, nodes: var seq[int]) =
    seen[i] = true
    nodes.add(i)
    for j in g[i]:
      if not seen[j]:
        dfs(j, nodes)
  for i in 0..<n:
    if not seen[i]:
      var nodes = newSeq[int]()
      dfs(i, nodes)
      cc.add(nodes)
  let k = cc.len
  if k == 1:
    echo 0
    return
  if (k - 1) * 2 > n:
    echo "Impossible"
    return
  var
    used = newSeqWith(n, false)
    ans: int64 = 0
  for nodes in cc:
    var mn = int64.high
    for i in nodes:
      mn = min(mn, a[i])
    for i in nodes:
      if mn == a[i]:
        used[i] = true
        ans += a[i]
        break
  var cost = newSeq[int64]()
  for i in 0..<n:
    if not used[i]:
      cost.add(a[i])
  cost.sort(cmp)
  ans += cost[0..<(k - 2)].sum
  echo ans
main()
