import strutils, sequtils, queues

var n: int
proc index(i, j: int): int =
  return min(i, j) * n + max(i, j)

proc main() =
  n = stdin.readLine.strip.parseInt
  let m = n * n
  var g = newSeqWith(m, newSeq[int]())
  for i in 0..<n:
    let a = stdin.readLine.strip.split.map(parseInt).mapIt(it - 1)
    for j in 0..<(n - 2):
      g[index(i, a[j])].add(index(i, a[j + 1]))

  var deg = newSeq[int](m)
  for i in 0..<m:
    for j in g[i]:
      deg[j] += 1
  var
    order = newSeq[int]()
    q = initQueue[int]()
  for i in 0..<m:
    if deg[i] == 0:
      q.enqueue(i)
  while q.len > 0:
    let i = q.dequeue
    order.add(i)
    for j in g[i]:
      deg[j] -= 1
      if deg[j] == 0:
        q.enqueue(j)
  if deg.anyIt(it > 0):
    echo "-1"
    return
  var dp = newSeqWith(m, 1)
  for i in order:
    for j in g[i]:
      dp[j] = max(dp[j], dp[i] + 1)
  echo dp.max
main()
