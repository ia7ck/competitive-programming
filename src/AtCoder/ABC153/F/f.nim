import strutils, sequtils, algorithm

type P = tuple[x, h: int64]
proc pcmp(p, q: P): int = cmp(p.x, q.x)

proc main() =
  var
    n: int
    d, a: int64
  let nda = stdin.readLine.strip.split.map(parseInt)
  (n, d, a) = (nda[0], nda[1].int64, nda[2].int64)
  var items = newSeq[P]()
  for i in 0..<n:
    var x, h: int64
    (x, h) = stdin.readLine.strip.split.map(parseBiggestInt)
    items.add((x, h))
  items.sort(proc(p, q: P): int = cmp(p.x, q.x))
  var acc = newSeq[int64](n + 1)
  var ans: int64 = 0
  for i in 0..<n:
    if i > 0:
      acc[i] += acc[i - 1]
    let
      x = items[i].x
      h = items[i].h - acc[i]
      c = (h + a - 1) div a
    if h <= 0 or c == 0:
      continue
    ans += c
    let
      j = items.lowerBound((x + d * 2 + 1, 0.int64), pcmp)
    acc[i] += a * c
    acc[j] -= a * c
  echo ans
main()
