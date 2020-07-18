import strutils, sequtils
proc printf(formatstr: cstring) {.importc: "printf", varargs,
  header: "<stdio.h>".}

proc f(n, a, b, c: int, memo: var seq[seq[seq[float64]]]): float64 =
  if memo[a][b][c]>=0:
    result = memo[a][b][c]
  else:
    if a==0 and b==0 and c==0:
      result = 0
    else:
      let s = a+b+c
      if a-1>=0:
        result+=f(n, a-1, b, c, memo)*(a/s)
      if b-1>=0:
        result+=f(n, a+1, b-1, c, memo)*(b/s)
      if c-1>=0:
        result+=f(n, a, b+1, c-1, memo)*(c/s)
      result+=(n/s)
    memo[a][b][c] = result

proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseInt)
  var
    cnt = newSeqWith(3, 0)
  for x in a:
    cnt[x-1]+=1
  var memo = newSeqWith(n+1, newSeqWith(n+1, newSeqWith(n+1, -1.0)))
  printf("%.18f\n", f(n, cnt[0], cnt[1], cnt[2], memo))
main()
