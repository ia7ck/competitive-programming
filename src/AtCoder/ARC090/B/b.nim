import strutils, sequtils, algorithm, queues

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt
  type E = tuple[to, cost: int]
  var g = newSeqWith(n, newSeq[E]())
  for i in 0..<m:
    let a, b, c = read().parseInt
    g[a - 1].add((b - 1, c))

  var deg = newSeq[int](n)
  for i in 0..<n:
    for e in g[i]:
      deg[e.to] += 1
  var
    d = newSeqWith(n, -1)
    q = initQueue[int]()
  for i in 0..<n:
    if deg[i] == 0:
      d[i] = 0
      q.enqueue(i)
  while q.len > 0:
    let i = q.dequeue
    for e in g[i]:
      if d[e.to] >= 0 and d[e.to] != d[i] + e.cost:
        echo "No"
        return
      if d[e.to] < 0:
        d[e.to] = d[i] + e.cost
        q.enqueue(e.to)
  if d.allIt(it in 0..1000000000):
    echo "Yes"
  else:
    echo "No"
main()
