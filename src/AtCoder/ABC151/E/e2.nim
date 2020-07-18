import strutils, sequtils, algorithm

const mo: int64 = 1000000007
proc mpow(a: int64, x: int64): int64 =
  if x == 0: return 1
  if x == 1: return a
  if x mod 2 == 1: return a * mpow(a, x - 1) mod mo
  return mpow(a * a mod mo, x div 2)

var fact, inv: seq[int64]
proc init(n: int) =
  fact = newSeq[int64](n + 1)
  inv = newSeq[int64](n + 1)
  fact[0] = 1
  for i in 1..n:
    fact[i] = fact[i - 1] * i mod mo
  inv[n] = mpow(fact[n], mo - 2)
  for i in countdown(n - 1, 0):
    inv[i] = inv[i + 1] * (i + 1) mod mo

proc binom(n, k: int): int64 =
  if n < 0 or k < 0: return 0
  if n < k: return 0
  return fact[n] * inv[k] mod mo * inv[n - k] mod mo

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt).sortedByIt(it)

  init(1_000_000)
  var ans: int64 = 0
  for i in 0..<n:
    if i >= k - 1:
      ans = (ans + a[i] * binom(i, k - 1) mod mo) mod mo
  for i in countdown(n - 1, 0):
    let j = n - i - 1
    if j >= k - 1:
      ans = (ans - a[i] * binom(j, k - 1) mod mo + mo) mod mo
  echo ans
main()
