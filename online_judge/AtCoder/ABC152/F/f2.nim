import strutils, sequtils, math

proc mpow(a, x, m: int64): int64 =
  if x == 0: return 1
  if x == 1: return a
  if x mod 2 == 0: return mpow(a * a mod m, x div 2, m)
  return a * mpow(a, x - 1, m) mod m

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseBiggestInt)
  type P = tuple[p, e: int]
  let m = a.max + 1
  var g = newSeq[int64](m)
  for it in a:
    var
      x = it
      f = newSeq[P]()
    for j in 2..it:
      if j * j > x: break
      if x mod j == 0:
        f.add((j.int, 0))
        while x mod j == 0:
          f[^1].e += 1
          x = x div j
    if x > 1:
      f.add((x.int, 1))
    for p, e in f.items:
      g[p] = max(g[p], e)
  const mo: int64 = 1_000_000_000 + 7
  var mul: int64 = 1
  for p, e in g:
    if e > 0:
      mul = mul * mpow(p, e, mo) mod mo
  var ans: int64 = 0
  for it in a:
    ans = (ans + mul * mpow(it, mo - 2, mo) mod mo) mod mo
  echo ans
main()
