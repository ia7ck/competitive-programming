import strutils, sequtils, queues
proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    edges = (0..<(n+m-1)).mapIt(stdin.readLine.split.map(parseInt))
  var in_deg = newSeq[int](n+1)
  for e in edges:
    in_deg[e[1]]+=1
  var root = 0
  for i in 1..n:
    if in_deg[i]==0:
      root = i
      break
  var par = newSeqWith(n+1, -1)
  par[root] = 0
  var g = newSeqWith(n+1, newSeq[int]())
  for e in edges:
    g[e[0]].add(e[1])
  var q = initQueue[int]()
  for chi in g[root]:
    in_deg[chi]-=1
    if in_deg[chi]==0:
      q.add(chi)
      par[chi] = root
  while q.len>0:
    let cur = q.dequeue
    for chi in g[cur]:
      in_deg[chi]-=1
      if in_deg[chi]==0:
        q.add(chi)
        par[chi] = cur
  for i in 1..n:
    doAssert(par[i]>=0)
  echo par[1..n].map(proc(p: int): string = $(p)).join("\n")
main()
