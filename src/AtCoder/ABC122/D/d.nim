import strutils, sequtils

proc agc(i, j, k: int): bool =
  if i == 0 and j == 2 and k == 1:
    return true
  if i == 0 and j == 1 and k == 2:
    return true
  if i == 2 and j == 0 and k == 1:
    return true
  return false

const mo = 1_000_000_000 + 7
proc add(a: var int, b: int) =
  a = (a + b) mod mo

proc main() =
  let n = stdin.readLine.strip.parseInt
  const m = 4
  var dp = newSeqWith(n + 1, newSeqWith(m, newSeqWith(m, newSeqWith(m, 0))))
  for i in 0..<m:
    for j in 0..<m:
      for k in 0..<m:
        dp[3][i][j][k] = if agc(i, j, k): 0 else: 1
  for t in 3..<n:
    for i in 0..<m:
      for j in 0..<m:
        for k in 0..<m:
          for y in 0..<m:
            if agc(j, k, y): continue # ?AGC, ?ACG, ?GAC
            if i == 0 and j == 2 and y == 1: continue # AG?C
            if i == 0 and k == 2 and y == 1: continue # A?GC
            add(dp[t + 1][j][k][y], dp[t][i][j][k])
  var res = 0
  for i in 0..<m:
    for j in 0..<m:
      for k in 0..<m:
        add(res, dp[n][i][j][k])
  echo res
main()
