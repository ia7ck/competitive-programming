import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

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

proc main() =
  let
    r, c = read().parseInt
    x, y = read().parseInt
    de, la = read().parseInt
    n = de + la

  # x 行 y 列の区画をひとつ固定して考える
  #   最後に (r - x + 1) * (c - y + 1) 倍すればよい
  # n 個の物 (デスク or ラック) の配置方法を求める
  #   最後に binom(n, de) 倍すればよい
  # [区画の上下左右の辺にそれぞれ 1 つ以上の物がある配置の仕方]
  # = [自由な配置の仕方] - [上下左右の辺のどれかに物がない配置の仕方]


  yieldCombination(30 * 30 + 1)
  var ans = binom(x * y, n)
  # たぶん if 文いらないです
  if x - 1 >= 1 and (x - 1) * y >= n:
    # 上に物がない
    # 下に物がない
    ans = ans - binom((x - 1) * y, n) * 2
  if y - 1 >= 1 and x * (y - 1) >= n:
    # 左に物がない
    # 右に物がない
    ans = ans - binom(x * (y - 1), n) * 2
  if x - 2 >= 1 and (x - 2) * y >= n:
    # 上下に物がない
    ans = ans + binom((x - 2) * y, n)
  if y - 2 >= 1 and x * (y - 2) >= n:
    # 左右に物がない
    ans = ans + binom(x * (y - 2), n)
  if x - 1 >= 1 and y - 1 >= 1 and (x - 1) * (y - 1) >= n:
    # 上・右に物がない
    # 上・左に物がない
    # 下・右に物がない
    # 下・左に物がない
    ans = ans + binom((x - 1) * (y - 1), n) * 4
  if x - 1 >= 1 and y - 2 >= 1 and (x - 1) * (y - 2) >= n:
    # 上・左右に物がない
    # 下・左右に物がない
    ans = ans - binom((x - 1) * (y - 2), n) * 2
  if x - 2 >= 1 and y - 1 >= 1 and (x - 2) * (y - 1) >= n:
    # 上下・右に物がない
    # 上下・左に物がない
    ans = ans - binom((x - 2) * (y - 1), n) * 2
  if x - 2 >= 1 and y - 2 >= 1 and (x - 2) * (y - 2) >= n:
    # 上下・左右に物がない
    ans = ans + binom((x - 2) * (y - 2), n)
  echo ans * binom(n, de) * (r - x + 1) * (c - y + 1)
main()
