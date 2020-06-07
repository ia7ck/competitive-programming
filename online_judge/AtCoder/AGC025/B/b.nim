import strutils

const mo: int64 = 998244353
type mint = distinct int64
proc toMint[Int](a: Int): mint =
  var res = a.int64 mod mo
  if res < 0:
    res = res + mo
  return res.mint
proc `+`(a, b: mint): mint = (a.int64 + b.int64).toMint
proc `-`(a, b: mint): mint = (a.int64 - b.int64).toMint
proc `*`(a, b: mint): mint = (a.int64 * b.int64).toMint
proc pow(a: mint, x: int64): mint =
  assert(x >= 0)
  var
    res = 1.toMint
    base = a
    exp = x
  while exp > 0:
    if (exp and 1) == 1:
      res = res * base
    base = base * base
    exp = exp div 2
  return res
proc inv(a: mint): mint = pow(a, mo - 2)
proc `div`(a, b: mint): mint = a * inv(b)
proc `$`(a: mint): string {.borrow.}

template yieldCombination(n: int = 20) =
  var
    fac {.inject.} = newSeq[mint](n)
    inv {.inject.} = newSeq[mint](n)
  fac[0] = 1.toMint
  for i in 1..<n:
    fac[i] = i.toMint * fac[i - 1]
  inv[n - 1] = 1.toMint div fac[n - 1]
  for i in countdown(n - 2, 0):
    inv[i] = inv[i + 1] * (i + 1).toMint
  let binom {.inject.} =
    proc(a, b: int): mint =
      if a < 0 or b < 0: return 0.toMint
      if a < b: return 0.toMint
      return fac[a] * inv[b] * inv[a - b]

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = read().parseBiggestInt
    b = read().parseBiggestInt
    k = read().parseBiggestInt
  yieldCombination(n + 1)
  var ans = 0.toMint
  for x in 0..n:
    if (k - x * a) mod b == 0:
      let y = (k - x * a) div b
      if 0 <= y and y <= n:
        ans = ans + binom(n, x) * binom(n, y.int)
  echo ans
main()
