import strutils

const mo: int64 = 1000000000 + 7
type mint = distinct int64
proc toMint[Int](a: Int): mint =
  var res = a.int64 mod mo
  if res < 0:
    res = res + mo
  return res.mint
proc `+`(a, b: mint): mint = (a.int64 + b.int64).toMint
proc `+=`(a: var mint, b: mint) = a = a + b
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
  let n = read().parseInt
  let a, b, c = read().parseInt.toMint

  yieldCombination(n * 2 + 1)
  # p: ゲームが進む確率
  # q: そのときに A が勝つ確率
  # r: そのときに A が負ける確率
  let
    p = (a + b) div 100.toMint
    q = a div (a + b)
    r = b div (a + b)
  var ans = 0.toMint
  for i in n..<(n * 2):
    # 引き分けを除いてちょうど i 回でゲームが終わる確率
    #   勝ち: (n - 1 回勝ち, i - n 回負け) + 最後に勝ち
    #   負け: (i - n 回勝ち, n - 1 回負け) + 最後に負け
    # 勝ち or 負けが 1 回出るまでに行うゲームの回数の期待値: 1/p
    ans += binom(i - 1, n - 1) * pow(q, n - 1) * pow(r, i - n) * q * i.toMint div p
    ans += binom(i - 1, i - n) * pow(q, i - n) * pow(r, n - 1) * r * i.toMint div p
  echo ans
main()
