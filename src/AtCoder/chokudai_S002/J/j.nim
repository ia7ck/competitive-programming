import strutils, sequtils, math, algorithm, future

proc f(x: int): seq[int] =
  var
    factors = newSeq[int]()
    i = 1
  while i * i <= x:
    if x mod i == 0:
      factors.add(i)
      factors.add(x div i)
    i += 1
  factors.sort((x, y) => -cmp(x, y))
  return factors

type P = tuple[a: int, b: int]
proc g(items: seq[P], cand: seq[int]): int =
  for x in cand:
    var ok = true
    for it in items:
      if gcd(x, it.a) != x and gcd(x, it.b) != x:
        ok = false
        break
    if ok:
      return x
  return 1

proc main() =
  let n = stdin.readLine.strip.parseInt
  var items = newSeq[P](n)
  for i in 0..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    items[i] = (a, b)
  var res = g(items, f(items[0].a))
  res = max(res, g(items, f(items[0].b)))
  echo res

main()
