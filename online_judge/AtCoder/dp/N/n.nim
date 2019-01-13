import strutils, sequtils

var
  n: int
  a: seq[int64]
  s: seq[int64]
  inf = high(int64) div 3
  mm: seq[seq[int64]]
proc f(i, j: int): int64 =
  if mm[i][j]<inf:
    result = mm[i][j]
  else:
    if i==j:
      result = 0
    else:
      result = inf
      for k in i..<j:
        # [i, k]と(k, j]をcombine
        result = min(result, f(i, k)+f(k+1, j)+(s[j]-s[i-1]))
    mm[i][j]=result

proc main() =
  n = stdin.readLine.parseInt
  a = stdin.readLine.split.map(parseBiggestInt)
  s = newSeqWith(n+1, 0'i64)
  for i in 0..<n:
    s[i+1] = s[i]+a[i]
  mm = newSeqWith(n+1, newSeqWith(n+1, inf))
  echo f(1, n)
main()
