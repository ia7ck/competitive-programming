import strutils, sequtils, algorithm

const mo: int64 = 1000000007
proc mpow(a: int64, x: int64): int64 =
  if x == 0: return 1
  elif x == 1: return a
  elif x mod 2 == 1: return a * mpow(a, x - 1) mod mo
  return mpow(a * a mod mo, x div 2)

var fact, inv: seq[int64]
proc binom(n, k: int): int64 =
  if n < 0 or k < 0: return 0
  if n < k: return 0
  return fact[n] * inv[k] mod mo * inv[n - k] mod mo

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var a = stdin.readLine.strip.split.map(parseBiggestInt)

  fact = newSeq[int64](n + 1)
  inv = newSeq[int64](n + 1)
  fact[0] = 1
  for i in 1..n:
    fact[i] = fact[i - 1] * i mod mo
  for i in 0..n:
    inv[i] = mpow(fact[i], mo - 2)
  a.sort(system.cmp)
  a.reverse
  var ans: int64 = 0
  for i in 0..<n:
    var t = a[i] * binom(n - i - 1, k - 1) mod mo
    if t < 0: t = t + mo
    ans = (ans + t) mod mo
  a.reverse
  for i in 0..<n:
    var t = a[i] * binom(n - i - 1, k - 1) mod mo
    if t < 0: t = t + mo
    ans = (ans - t + mo) mod mo
  echo ans
main()
