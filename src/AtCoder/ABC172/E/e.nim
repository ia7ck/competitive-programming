import strutils

const mo: int64 = 1000000007
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
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt

  yieldCombination(max(n, m) + 1)
  var ans = 0.toMint
  ans = ans + pow(binom(m, n) * fac[n], 2)
  for i in 1..n:
    let a = binom(m, i) * binom(n, i) * fac[i] * pow(binom(m - i, n - i) * fac[n - i], 2)
    if i mod 2 == 1:
      ans = ans - a
    else:
      ans = ans + a
  echo ans
main()
