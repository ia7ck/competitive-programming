import strutils, sequtils, algorithm

proc main() =
  var n, a, m: int64
  (n, a, m) = stdin.readLine.strip.split.map(parseBiggestInt)
  var di = stdin.readLine.strip.split.map(parseBiggestInt)

  di.sort(system.cmp)
  var
    last: int64 = 0
    s = newSeq[int64]()
  for d in di:
    if last + a < d:
      s.add(d - (last + a))
    last = d
  if last + a <= n:
    s.add(n - (last + a) + 1)
  var cost = m
  for it in s:
    cost += (it + a - 1) div a
  echo n - cost
main()
