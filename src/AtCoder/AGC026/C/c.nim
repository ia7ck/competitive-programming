import 
  strutils, # parseInt
  sequtils, # map
  future, # dump
  tables, # table
  algorithm # reverse

var 
  n=stdin.readLine.parseInt
  s=stdin.readLine
  hash=initTable[(string, string), int]()

for bit in 0..<(1 shl n):
  var 
    red=""
    blue=""
  for i in 0..<n:
    if (bit and (1 shl i))>0:
      red.add(s[i])
    else:
      blue.add(s[i])
  if hash.hasKey((red, blue)):
    hash[(red, blue)]+=1
  else:
    hash[(red, blue)]=1

var 
  t=s[n..<(n*2)]
  cnt=0'i64
reverse(t)


for bit in 0..<(1 shl n):
  var 
    red=""
    blue=""
  for i in 0..<n:
    if (bit and (1 shl i))>0:
      red.add(t[i])
    else:
      blue.add(t[i])
  if hash.hasKey((blue, red)):
    cnt+=hash[(blue, red)]

echo cnt