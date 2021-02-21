import strutils, sequtils

let inf = high(int64)
var memo: seq[seq[int64]]
proc f(a: seq[int64], l, r, t: int): int64 =
  if memo[l][r] > -inf:
    result = memo[l][r]
  else:
    if l==r:
      result = if t == 1: a[r] else: -a[r]
    else:
      let
        v1 = f(a, l+1, r, 1-t)
        v2 = f(a, l, r-1, 1-t)
      if t==1:
        result = max(v1+a[l], v2+a[r])
      else:
        result = min(v1-a[l], v2-a[r])
    memo[l][r] = result
proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseBiggestInt)
  memo = newSeqWith(n, newSeqWith(n, -inf))
  echo f(a, 0, n-1, 1)
main()
