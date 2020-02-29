import strutils, sequtils

const mo: int64 = 1000000000 + 7
proc mpow(a, x: int64): int64 =
  if x == 0: return 1
  if x == 1: return a
  if x mod 2 == 0: return mpow(a * a mod mo, x div 2)
  return mpow(a, x - 1) * a mod mo

proc mdiv(a, b: int64): int64 =
  return a mod mo * mpow(b mod mo, mo - 2) mod mo

proc main() =
  var n, m, k, p, q: int64
  (n, m, k, p, q) = stdin.readLine.strip.split.map(parseBiggestInt)
  let a = newSeqWith(n.int, stdin.readLine.strip.parseBiggestInt)

  let
    s = (1 - mdiv(p, q) * 2 mod mo + mo) mod mo
    t = mpow(s, k)
    u = t * mdiv(1, 2) mod mo
    v = t * (-mdiv(1, 2) + mo) mod mo
  var ans: int64 = 0
  for i in 0..<a.len:
    let w = if i < m: u else: v
    ans = (ans + (w + mdiv(1, 2)) mod mo * a[i] mod mo) mod mo
  echo ans
main()
