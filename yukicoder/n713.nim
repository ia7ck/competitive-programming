import strutils, sequtils

var n=stdin.readLine.parseInt

proc isPrime(x: int): bool=
  if x==1:
    return false
  elif x==2:
    return true
  else:
    for i in 2..x:
      if i*i>x:
        break
      if x mod i==0:
        return false
    return true

var ps=filter(toSeq(1..n)) do(x: int)->bool: isPrime(x)
echo ps.foldl(a+b, 0)