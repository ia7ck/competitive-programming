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

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var dp = newSeq[mint](k + 1)
  dp[0] = 1.toMint
  for i in 0..<n:
    var nxt = newSeq[mint](k + 1)
    for j in 0..k:
      nxt[j] = dp[j]
      if j - a[i] - 1 >= 0:
        nxt[j] = nxt[j] - dp[j - a[i] - 1]
    for j in 0..<k:
      nxt[j + 1] = nxt[j + 1] + nxt[j]
    dp.swap(nxt)
  echo dp[k]
main()
