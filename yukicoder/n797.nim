import strutils, sequtils

proc modpow(a, x, mo: int64): int64 =
  if x == 0:
    return 1
  elif x == 1:
    return a
  elif x mod 2 == 0:
    return modpow(a * a mod mo, x div 2, mo)
  else:
    return a * modpow(a, x - 1, mo) mod mo

proc cmb(n, k: int, fact: seq[int64], mo: int64): int64 =
  if n < k:
    return 0
  elif k == 0:
    return 1
  elif n == k:
    return 1
  else:
    result = fact[n]
    result = result * modpow(fact[k], mo - 2, mo) mod mo
    result = result * modpow(fact[n - k], mo - 2, mo) mod mo
    return result

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
    mo = 1000000000 + 7
  var fact = newSeq[int64](n + 1)
  fact[0] = 1
  for i in 1..n:
    fact[i] = fact[i - 1] * i mod mo
  var ans: int64 = 0
  for k, it in a:
    ans = (ans + cmb(n - 1, k, fact, mo) * it) mod mo
  echo ans
main()
