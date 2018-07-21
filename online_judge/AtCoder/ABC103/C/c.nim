import strutils, sequtils

var 
  n=stdin.readLine.parseInt
  a=stdin.readLine.split.map(parseInt)
  sum=0

for e in a: sum+=e-1
echo sum