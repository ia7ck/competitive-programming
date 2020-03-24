import strutils, sequtils, queues

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, newSeqWith(n - 1, read().parseInt))

  proc index(i, j: int): int =
    return min(i, j) * n + max(i, j)
  var g = newSeqWith(n * n, newSeq[int]())
  for x in 0..<n:
    for j in 0..<(a[x].len - 1):
      g[index(x, a[x][j] - 1)].add(index(x, a[x][j + 1] - 1))
  var deg = newSeq[int](g.len)
  for i in 0..<g.len:
    for j in g[i]:
      deg[j] += 1
  var
    q = initQueue[int]()
    ord = newSeq[int]()
  for i in 0..<g.len:
    if deg[i] == 0:
      q.enqueue(i)
      ord.add(i)
  while q.len > 0:
    let i = q.dequeue
    for j in g[i]:
      deg[j] -= 1
      if deg[j] == 0:
        q.enqueue(j)
        ord.add(j)
  if deg.anyIt(it > 0):
    echo "-1"
    return
  var dp = newSeqWith(g.len, 1)
  for i in ord:
    for j in g[i]:
      dp[j] = max(dp[j], dp[i] + 1)
  echo dp.max
main()
