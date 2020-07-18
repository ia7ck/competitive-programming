import strutils, sequtils
var 
  args=stdin.readLine.split.map(parseInt)
  (n, m)=(args[0], args[1])

var
  deg=newSeq[int](n)
  stack=newSeqWith(m, newSeq[int]())
  top=newSeq[int](m)
  cnt=0

for i in 0..<n:
  var
    params=stdin.readLine.split.map(parseInt)
    ad=false
  for j, val in params:
    if top[j]<val:
      while stack[j].len>0:
        var k=stack[j].pop
        deg[k]-=1
        if deg[k]==0:
          cnt-=1
      stack[j].add(i)
      deg[i]+=1
      top[j]=val
      ad=true
    elif top[j]==val:
      stack[j].add(i)
      deg[i]+=1
      ad=true
    # else:
  if ad:
    cnt+=1
  echo cnt