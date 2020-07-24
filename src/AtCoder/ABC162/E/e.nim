import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

const mo: int64 = 1000000000 + 7
proc mpow(a, x: int64): int64 =
  if x == 1:
    return a
  if x mod 2 == 0:
    return mpow(a * a mod mo, x div 2)
  return mpow(a, x - 1) * a mod mo

proc main() =
  let n, k = read().parseInt

  var
    dp = newSeq[int64](k + 1)
    ans: int64 = 0
  for i in countdown(k, 1):
    dp[i] = mpow(k div i, n)
    var j = i * 2
    while j <= k:
      dp[i] = (dp[i] - dp[j] + mo) mod mo
      j += i
    ans = (ans + dp[i] * i mod mo) mod mo
  echo ans
main()
