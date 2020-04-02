import strutils, sequtils

const mo: int64 = 1000000000 + 7
type mint = distinct int64
proc toMint[Int](a: Int): mint =
  var res = a.int64 mod mo
  if res < 0:
    res = res + mo
  return res.mint
proc `+`[IntA, IntB](a: IntA, b: IntB): mint = (a.int64 + b.int64).toMint
proc `-`[IntA, IntB](a: IntA, b: IntB): mint = (a.int64 - b.int64).toMint
proc `*`[IntA, IntB](a: IntA, b: IntB): mint = (a.int64 * b.int64).toMint
proc pow[Int](a: Int, x: int64): mint =
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
proc inv[Int](a: Int): mint = pow(a, mo - 2)
proc `div`[IntA, IntB](a: IntA, b: IntB): mint = a * inv(b)
proc `$`(a: mint): string {.borrow.}

template yieldCombination(n: int = 20) =
  var
    fac {.inject.} = newSeq[mint](n)
    inv {.inject.} = newSeq[mint](n)
  fac[0] = 1.toMint
  for i in 1..<n:
    fac[i] = i * fac[i - 1]
  inv[n - 1] = 1 div fac[n - 1]
  for i in countdown(n - 2, 0):
    inv[i] = inv[i + 1] * (i + 1)
  let binom {.inject.} =
    proc(a, b: int): mint =
      if a < 0 or b < 0: return 0.toMint
      if a < b: return 0.toMint
      return fac[a] * inv[b] * inv[a - b]

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, k = read().parseInt

  yieldCombination(n * 2)
  var ans = 0.toMint
  for i in 0..min(n - 1, k): # 空の部屋が i 個
    # a[1] + a[2] + ... + a[n - i] = n
    # a[1], a[2], ..., a[n - i] ≧ 1
    # ⇔
    # a[1] + a[2] + ... + a[n - i] = n - (n - i) = i
    # a[1], a[2], ..., a[n - i] ≧ 0
    ans = ans + binom(i + (n - i - 1), i) * binom(n, i)
  echo ans
main()
