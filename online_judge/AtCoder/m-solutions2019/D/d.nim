import strutils, sequtils, queues, algorithm, math

proc main() =
  let n = stdin.readLine.strip.parseInt
  var
    g = newSeqWith(n, newSeq[int]())
    deg = newSeq[int](n)
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    a -= 1
    b -= 1
    g[a].add(b)
    g[b].add(a)
    deg[a] += 1
    deg[b] += 1
  var c = stdin.readLine.strip.split.map(parseInt)
  if n == 1:
    echo 0
    echo c[0]
    return

  var ans = newSeq[int](n)
  c.sort(system.cmp)
  echo c[0..<(^1)].sum
  var
    q = initQueue[int]()
    seen = newSeq[bool](n)
  for i in 0..<n:
    if deg[i] == 1:
      ans[i] = c.pop
      seen[i] = true
      q.enqueue(i)
      break
  while q.len > 0:
    let cur = q.dequeue
    for nxt in g[cur]:
      if not seen[nxt]:
        doAssert(c.len > 0)
        ans[nxt] = c.pop
        seen[nxt] = true
        q.enqueue(nxt)
  doAssert(ans.allIt(it > 0))
  echo ans.mapIt($it).join(" ")
main()
