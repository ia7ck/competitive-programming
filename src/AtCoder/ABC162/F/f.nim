import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc chmax(a: var int64, b: int64) =
  if a < b: a = b

proc main() =
  let n = read().parseInt
  var a = newSeqWith(n, read().parseBiggestInt)
  a.add(0)
  a.add(0)
  let m = if n mod 2 == 0: 2 else: 3
  const inf = int64.high div 2
  var dp = newSeqWith(n + 10, newSeqWith(m, -inf))
  for j in 0..<m:
    dp[0][j] = 0
  for i in 0..<n:
    # echo dp[i]
    for j in 0..<m:
      chmax(dp[i + 2][j], dp[i][j] + a[i])
      if j >= 1:
        chmax(dp[i + 3][j - 1], dp[i][j] + a[i + 1])
      if j >= 2:
        chmax(dp[i + 4][j - 2], dp[i][j] + a[i + 2])
  # echo dp[n..^1]
  var ans = -inf
  for r in dp[n..^1]:
    # echo r
    chmax(ans, r.max)
  doAssert(ans > -inf)
  echo ans
main()
