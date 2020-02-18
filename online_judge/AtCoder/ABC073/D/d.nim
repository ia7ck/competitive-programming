import strutils, sequtils, algorithm

proc main() =
  var n, m, k: int
  (n, m, k) = stdin.readLine.strip.split.map(parseint)
  var r = stdin.readLine.strip.split.map(parseint).sortedByIt(it)
  const inf = 1234567890
  var d = newSeqWith(n, newSeqWith(n, inf))
  for i in 0..<n:
    d[i][i] = 0
  for i in 0..<m:
    var a, b, c: int
    (a, b, c) = stdin.readLine.strip.split.map(parseint)
    d[a - 1][b - 1] = c
    d[b - 1][a - 1] = c
  for i in 0..<n:
    for s in 0..<n:
      for t in 0..<n:
        d[s][t] = min(d[s][t], d[s][i] + d[i][t])
  var ans = inf
  while true:
    var s = 0
    for i in 1..<k:
      s += d[r[i - 1] - 1][r[i] - 1]
    ans = min(ans, s)
    if not r.nextPermutation:
      break
  echo ans
main()
