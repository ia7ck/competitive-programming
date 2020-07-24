import strutils, sequtils, algorithm

var 
  a=stdin.readLine.split.map(parseInt)
  idx= @[0, 1, 2]
  mn=10000

while idx.nextPermutation:
  mn=min(mn, abs(a[idx[0]]-a[idx[1]])+abs(a[idx[1]]-a[idx[2]]))

echo mn