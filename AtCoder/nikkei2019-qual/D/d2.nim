import strutils, sequtils, queues

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseint)
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<(n - 1 + m):
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseint)
    g[a - 1].add(b - 1)
  var deg = newSeq[int](n)
  for i in 0..<n:
    for j in g[i]:
      deg[j] += 1
  var
    par = newSeqWith(n, -1)
    q = initQueue[int]()
  let root = (0..<n).mapIt(it).filterIt(deg[it] == 0)
  doAssert root.len == 1
  par[root[0]] = 0
  q.enqueue(root[0])
  while q.len > 0:
    let i = q.dequeue
    for j in g[i]:
      deg[j] -= 1
      if deg[j] == 0:
        par[j] = i + 1
        q.enqueue(j)
  doAssert par.allIt(it >= 0)
  echo par.mapIt($it).join("\n")
main()
