import strutils, sequtils

var
  n=stdin.readLine.parseInt
  freq=newSeq[int](2000)

for _ in 0..<n:
  freq[stdin.readLine.len-2]+=1

var mx=freq.max
for x in countdown(1000, 0):
  if freq[x]==mx:
    echo x
    break
