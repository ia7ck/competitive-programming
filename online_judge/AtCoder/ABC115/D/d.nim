import strutils, sequtils

proc pattyCnt(n: int): int64 =
  if n==0:
    result = 1
  else:
    result = pattyCnt(n-1)*2+1

proc burgerLen(n: int): int64 =
  if n==0:
    result = 1
  else:
    result = burgerLen(n-1)*2+3

proc f(n: int, x: int64): int64 =
  if x==0:
    result = 0
  elif n==0:
    result = 1
  else:
    var y = x-1
    if y>=burgerLen(n-1):
      result+=pattyCnt(n-1)
    else:
      return f(n-1, y)
    y-=burgerLen(n-1)
    if y>=1:
      result+=1
      y-=1
      if y>=burgerLen(n-1):
        result+=pattyCnt(n-1)
      else:
        return result+f(n-1, y)


proc main() =
  let
    nx = stdin.readLine.split.map(parseInt)
    (n, x) = (nx[0], nx[1])
  echo f(n, x)

main()
