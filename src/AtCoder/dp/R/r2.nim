import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

const mo: int64 = 1000000000 + 7
proc matmult(a, b: seq[seq[int64]]): seq[seq[int64]] =
  let
    n = a.len
    m = b[0].len
  var c = newSeqWith(n, newSeq[int64](n))
  for i in 0..<n:
    for j in 0..<m:
      for k in 0..<n:
        c[i][j] = (c[i][j] + a[i][k] * b[k][j] mod mo) mod mo
  return c

proc pow(a: seq[seq[int64]], k: int64): seq[seq[int64]] =
  if k == 1:
    return a
  if k mod 2 == 0:
    return pow(matmult(a, a), k div 2)
  else:
    return matmult(a, pow(a, k - 1))

proc main() =
  let
    n = read().parseInt
    k = read().parseBiggestInt
    a = newSeqWith(n, newSeqWith(n, read().parseBiggestInt))
  let
    b = pow(a, k)
    v = matmult(b, newSeqWith(n, @[1.int64]))
  var ans: int64 = 0
  for r in v:
    ans = (ans + r[0]) mod mo
  echo ans
main()
