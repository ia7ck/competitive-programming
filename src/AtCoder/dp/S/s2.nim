import strutils, sequtils, math

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

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    k = read()
    d = read().parseInt

  var dp = newSeqWith(k.len + 1, newSeqWith(2, newSeqWith(d, 0.toMint)))
  dp[k.len][0][0] = 1.toMint
  for i in countdown(k.len - 1, 0):
    for j in 0..<2:
      for s in 0..<d:
        for x in 0..9:
          let y = k[i].ord - '0'.ord
          var nj: int
          if x < y:
            nj = 1
          elif x > y:
            nj = 0
          else:
            nj = j
          let  ns = (s + x) mod d
          dp[i][nj][ns] = dp[i][nj][ns] + dp[i + 1][j][s]
  var ans = dp[0][1][0] - 1
  if k.mapIt((it.ord - '0'.ord).int).sum mod d == 0:
    ans = ans + 1
  echo ans
main()
