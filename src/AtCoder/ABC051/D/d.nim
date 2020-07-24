import strutils, sequtils, algorithm

type Edge=tuple[u: int, v: int, cost: int]
var
  args=stdin.readLine.split.map(parseInt)
  (n, m)=(args[0], args[1])
  edges=newSeq[Edge](m)
  d=newSeqWith(n, newSeqWith(n, 1000000000))
for i in 0..<n: d[i][i]=0
for i in 0..<m:
  args=stdin.readLine.split.map(parseInt)
  var (a, b, c)=(args[0], args[1], args[2])
  edges[i]=(u: a-1, v: b-1, cost: c)
  d[a-1][b-1]=c
  d[b-1][a-1]=c
for m in 0..<n:
  for i in 0..<n:
    for j in 0..<n:
      d[i][j]=min(d[i][j], d[i][m]+d[m][j])
var cnt=0
for e in edges:
  var used=false
  for i in 0..<n:
    for j in 0..<n:
      if d[i][j]==d[i][e.u]+e.cost+d[e.v][j]:
        used=true
  if used==false: cnt+=1
echo cnt
