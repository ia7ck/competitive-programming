import strutils, sequtils

const mo: int64 = 1000000000 + 7
proc mpow(a, x: int64): int64 =
  if x == 0: return 1
  if x == 1: return a
  if x mod 2 == 0: return mpow(a * a mod mo, x div 2)
  return mpow(a, x - 1) * a mod mo

proc binom(n, k: int64): int64 =
  result = 1
  for i in 1..k:
    result = result * (n - i + 1) mod mo
    result = result * mpow(i, mo - 2) mod mo

proc main() =
  var n, a, b: int64
  (n, a, b) = stdin.readLine.strip.split.map(parseBiggestInt)
  var ans = mpow(2, n) - 1
  ans = (ans - binom(n, a) + mo) mod mo
  ans = (ans - binom(n, b) + mo) mod mo
  echo ans
main()
