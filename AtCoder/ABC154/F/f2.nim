import strutils, sequtils

const mo: int64 = 1000000000 + 7
proc mpow(a, x: int64): int64 =
  var
    res:int64 = 1
    b = a
    e = x
  while e > 0:
    if (e and 1) == 1:
      res = res * b mod mo
    b = b * b mod mo
    e = e div 2
  return res

# sum_{i = 0, 1, ..., r} sum_{j = 0, 1, ..., c} f(i, j)
proc solve(r, c: int): int64 =
  let n = r + c + 10
  var
    fac = newSeq[int64](n)
    inv = newSeq[int64](n)
  fac[0] = 1
  for i in 1..<n:
    fac[i] = i * fac[i - 1] mod mo
  inv[n - 1] = mpow(fac[n - 1], mo - 2)
  for i in countdown(n - 2, 0):
    inv[i] = inv[i + 1] * (i + 1) mod mo
  proc binom(a, b: int): int64 =
    if a < 0 or b < 0: return 0
    if a < b: return 0
    return fac[a] * inv[a - b] mod mo * inv[b] mod mo
  proc countPath(a, b: int): int64 =
    return binom(a + b, b)
  var res: int64 = 0
  for i in 0..r:
    res = (res + countPath(i + 1, c)) mod mo
  return res

proc main() =
  var r1, c1, r2, c2: int
  (r1, c1, r2, c2) = stdin.readLine.strip.split.map(parseInt)

  var ans = solve(r2, c2)
  ans = (ans - solve(r1 - 1, c2) + mo) mod mo
  ans = (ans - solve(r2, c1 - 1) + mo) mod mo
  ans = (ans + solve(r1 - 1, c1 - 1) + mo) mod mo
  echo ans
main()
