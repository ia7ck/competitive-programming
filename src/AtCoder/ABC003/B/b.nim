import strutils, sequtils

let
  s = stdin.readLine
  t = stdin.readLine
  pat = @['a', 't', 'c', 'o', 'd', 'e', 'r', '@']

var ok = true
for i in 0..<len(s):
  if s[i]=='@':
    let a = pat.filter(proc(it: char): bool = it==t[i]).len
    if a==0:
      ok = false
  elif t[i]=='@':
    let a = pat.filter(proc(it: char): bool = it==s[i]).len
    if a==0:
      ok = false
  elif s[i]!=t[i]:
    ok = false

if ok:
  echo "You can win"
else:
  echo "You will lose"
