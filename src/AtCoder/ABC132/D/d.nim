import strutils, sequtils

const mo: int64 = 1000000007
proc mpow(a: int64, x: int64): int64 =
  if x == 0:
    return 1
  elif x == 1:
    return a
  elif x mod 2 == 0:
    return mpow(a * a mod mo, x div 2)
  else:
    return a * mpow(a, x - 1) mod mo

var fact, fact_inv: seq[int64]
proc binom(n, k: int): int64 =
  if n < k:
    return 0
  return ((fact[n] * fact_inv[k] mod mo) * fact_inv[n - k] mod mo)

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)

  let m = 5000
  fact = newSeq[int64](m)
  fact_inv = newSeq[int64](m)
  fact[0] = 1
  for i in 1..<m:
    fact[i] = fact[i - 1] * i mod mo
  for i in 0..<m:
    fact_inv[i] = mpow(fact[i], mo - 2)

  var ans = newSeq[int64](k)
  for i in 1..k:
    let
      c1 = binom(k - 1, i - 1)
      c2 = binom(n - k + 1, i)
    ans[i - 1] = c1 * c2 mod mo
  echo ans.mapIt($it).join("\n")
main()
