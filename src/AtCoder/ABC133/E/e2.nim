import strutils, sequtils

const mo: int64 = 1000000000 + 7
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
  let n, k = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  yieldCombination(max(k, n) + 1)
  proc dfs(i, p: int, a: var mint)=
    let ch = g[i].filterIt(it != p).len
    if ch >= 1:
      if p >= 0:
        a = a * binom(k - 2, ch) * fac[ch]
      else:
        a = a * binom(k - 1, ch) * fac[ch]
    # echo i, " ", a
    for j in g[i]:
      if j != p:
        dfs(j, i, a)
  var a = k.toMint
  dfs(0, -1, a)
  echo a
main()
