import strutils, sequtils, algorithm, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  var
    x, y, a, b, c = read().parseInt
    p = newSeqWith(a, read().parseBiggestInt)
    q = newSeqWith(b, read().parseBiggestInt)
    r = newSeqWith(c, read().parseBiggestInt)
  p.sort(cmp)
  q.sort(cmp)
  let
    i = a - x
    j = b - y
  var
    ans = p[i..<a].sum + q[j..<b].sum
    vals = concat(p[i..<a], q[j..<b])
  vals.sort(cmp)
  r.sort(cmp)
  r.reverse
  var k = 0
  for it in r:
    if k < vals.len and vals[k] < it:
      ans -= vals[k]
      ans += it
      k += 1
  echo ans
main()

