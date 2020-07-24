import strutils, sequtils

const mo: int64 = 1000000007
proc mpow(a: int64, x: int64): int64 =
  if x == 0: return 1
  elif x == 1: return a
  elif x mod 2 == 1: return a * mpow(a, x - 1) mod mo
  return mpow(a * a mod mo, x div 2)

var fact, inv: seq[int64]
proc init(n: int) =
  fact = newSeq[int64](n + 1)
  inv = newSeq[int64](n + 1)
  fact[0] = 1
  for i in 1..n:
    fact[i] = fact[i - 1] * i mod mo
  for i in 0..n:
    inv[i] = mpow(fact[i], mo - 2)

proc binom(n, k: int): int64 =
  if n < 0 or k < 0: return 0
  if n < k: return 0
  return fact[n] * inv[k] mod mo * inv[n - k] mod mo

# a_1 + a_2 + ... + a_n = s
# a_i >= 1
# a_i <= a_{i+1}
# になる a の個数
proc f(n, s: int): int64 =
  let t = s - n
  if t < 0: return 0
  return fact[t + (n - 1)] * inv[t] mod mo * inv[n - 1] mod mo

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)

  init(1_000_000)

  var ans = 0
  for i in 0..min(n - 1, k):
    ans = (ans + f(n - i, n) * binom(n, n - i) mod mo) mod mo
  echo ans
main()
