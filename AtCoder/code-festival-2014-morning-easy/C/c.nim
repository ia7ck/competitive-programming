import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, m = read().parseInt
    s, t = read().parseInt
  const inf: int = 1000000000
  var d = newSeqWith(n, newSeqWith(n, inf))
  for i in 0..<n:
    d[i][i] = 0
  for i in 0..<m:
    let x, y, w = read().parseInt
    d[x - 1][y - 1] = w
    d[y - 1][x - 1] = w
  for k in 0..<n:
    for i in 0..<n:
      for j in 0..<n:
        d[i][j] = min(d[i][j], d[i][k] + d[k][j])
  for u in 0..<n:
    if d[u][s - 1] < inf:
      if d[u][s - 1] == d[u][t - 1]:
        echo u + 1
        return
  echo -1
main()
