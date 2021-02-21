import strutils, sequtils, algorithm, future

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseBiggestInt)
  type P = tuple[val: int64, cnt: int]
  var items = newSeq[P]()
  for it in a:
    items.add((it, 1))
  for i in 0..<m:
    var b, c: int
    (b, c) = stdin.readLine.strip.split.map(parseInt)
    items.add((c.int64, b))
  items.sort((x, y) => -cmp(x.val, y.val))
  var ans: int64 = 0
  for it in items:
    let sub = min(n, it.cnt)
    ans += it.val * sub
    n -= sub
    if n == 0:
      break
  doAssert(n == 0)
  echo ans
main()
