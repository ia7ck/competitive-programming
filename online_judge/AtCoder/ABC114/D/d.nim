import strutils, sequtils

proc f(N: int): seq[int] =
  result = newSeq[int]()
  var
    n = N
    b = 2
  while b*b<=n:
    while n mod b == 0:
      n = n div b
      result.add(b)
    b+=1
  if n>1:
    result.add(n)

proc main() =
  let n = stdin.readLine.parseInt
  var exp = newSeqWith(n+1, 1)
  for i in 1..n:
    let fac = f(i)
    for e in fac:
      exp[e]+=1
  var
    three = 0
    five = 0
    fifteen = 0
    twenty_five = 0
    seventy_five = 0
  for c in exp:
    if 3<=c:
      three+=1
    if 5<=c:
      five+=1
    if 15<=c:
      fifteen+=1
    if 25<=c:
      twenty_five+=1
    if 75<=c:
      seventy_five+=1
  var ans = 0
  ans+=(three-2)*five*(five-1) div 2 # 3 5 5
  ans+=(three-1)*twenty_five  # 3 25
  ans+=fifteen*(five-1)       # 15 5
  ans+=seventy_five           # 75
  echo ans
main()
