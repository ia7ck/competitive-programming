import strutils, sequtils

var
  args=stdin.readLine.split.map(parseInt)
  (n, x)=(args[0], args[1])
  a=stdin.readLine.split.map(parseInt)

var freq=newSeq[int64](100000+1)
for v in a: freq[v]+=1
var tot:int64=0
for v in a:
  if 0<=x-v and x-v<=100000:
    tot+=freq[x-v]
echo tot

