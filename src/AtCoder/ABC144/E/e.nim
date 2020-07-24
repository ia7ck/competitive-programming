import strutils, sequtils, algorithm, math

proc main() =
  var nn, k: int64
  (nn, k) = stdin.readLine.strip.split.map(parseBiggestInt)
  let n = nn.int
  var
    a = stdin.readLine.strip.split.map(parseInt)
    b = stdin.readLine.strip.split.map(parseInt)
  a.sort(system.cmp)
  b.sort(system.cmp)
  b.reverse

  if a.sum <= k:
    echo 0
    return

  var
    ok = a.max * b.max
    ng = 0
  while ok - ng > 1:
    let m = (ok + ng) div 2
    let c = b.mapIt(m div it)
    var d: int64 = 0
    for i in 0..<n:
      d += max(0, a[i] - c[i])
    if d <= k:
      ok = m
    else:
      ng = m
  echo ok
main() #

# K = 14
# 1 1 2 3 3 4 5 5 5 6 9
# 9 9 8 8 7 6 4 3 3 2 2

# 12/x
# 1 1 1 1 1 2 3 4 4 6 6
