import strutils, sequtils

proc main()=
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  let
    d = newSeqWith(m, stdin.readLine.strip.split.map(parseInt))
    c = newSeqWith(n, stdin.readLine.strip.split.map(parseInt))

  var cost = newSeqWith(m, newSeq[int](3))
  for i in 0..<m:
    for y in 0..<n:
      for x in 0..<n:
        cost[i][(y + x) mod 3] += d[c[y][x] - 1][i]
  var ans = 9876543210
  for i in 0..<m:
    for j in 0..<m:
      for k in 0..<m:
        if i == j or j == k or k == i:
          continue
        ans = min(ans, cost[i][0] + cost[j][1] + cost[k][2])
  echo ans
main()
