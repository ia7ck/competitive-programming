import strutils, sequtils, algorithm

var
  s=stdin.readLine
  res=0
  i=s.len-1

while i>=0:
  if s[i]==')' or s[i].isDigit:
    var sub=0
    if s[i]==')':
      i-=1
      while s[i]!='(':
        if s[i].isDigit:
          var sgn=if i-1>=0 and s[i-1]=='-': -1 else : 1
          sub+=(s[i].ord-'0'.ord)*sgn
        i-=1
    elif s[i].isDigit:
      sub=s[i].ord-'0'.ord
    var sgn=if i-1>=0 and s[i-1]=='-': -1 else : 1  
    res+=sub*sgn
  i-=1

echo res