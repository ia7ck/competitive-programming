import strutils, sequtils

proc main() =
  var
    d: int64 = stdin.readLine.parseInt
    k = 1
    n = 9'i64
  while d-n*k>0:
    d-=n*k
    n*=10
    k+=1
  doAssert(d>0)               # ?
  let
    m = $(n div 9 + (d-1) div k)
    j = int((d-1) mod k)
  echo m[j]

main()
