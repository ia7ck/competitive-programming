import strutils, sequtils, algorithm, queues

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  type E = tuple[to, cost: int]
  var
    g = newSeqWith(n, newSeq[E]())
    in_deg = newSeq[int](n)
  for i in 0..<m:
    var s, t, d: int
    (s, t, d) = stdin.readLine.strip.split.map(parseInt)
    g[s - 1].add((t - 1, d))
    in_deg[t - 1] += 1
  var d = newSeq[int](n)
  fill(d, -1)

  let check = proc(s: int): bool =
    var q = initQueue[int]()
    q.enqueue(s)
    d[s] = 0
    while q.len > 0:
      let v = q.dequeue
      for e in g[v]:
        if d[e.to] == -1:
          q.enqueue(e.to)
          d[e.to] = d[v] + e.cost
        elif d[e.to] != d[v] + e.cost:
          return false
    return true

  for s in 0..<n:
    if in_deg[s] == 0:
      let ok = check(s)
      if not ok:
        echo "No"
        return
  if d.allIt(it >= 0):
    echo "Yes"
  else:
    echo "No"
main()
