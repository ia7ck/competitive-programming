import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

# 半時計回りに s まで取って
# 時計回りに t まで取る
proc solve(n: int, c: int64, x, v: seq[int64]): int64 =
  var
    cv = newSeq[int64](n)
    energy = newSeq[int64](n)
  for i in 0..<n:
    cv[i] = v[i]
    if i > 0:
      cv[i] += cv[i - 1]
    energy[i] = cv[i] - x[i]
  # dp[i]: max(energy[0], ..., energy[i])
  var dp = newSeq[int64](n)
  for i in 0..<n:
    dp[i] = energy[i]
    if i > 0:
      dp[i] = max(dp[i], dp[i - 1])
  var ans = dp[n - 1]
  for s in countdown(n - 1, 1):
    let
      d = c - x[s] # 初期位置 --> s までの距離
      v1 = cv[n - 1] - cv[s] + v[s] # v[i] + v[i + 1] + ... + v[n - 1]
    ans = max(ans, v1 - d * 2 + dp[s - 1])
  return ans

proc main() =
  let (n, c) = (read().parseInt, read().parseBiggestInt)
  var sushi = newSeqWith(n, (x: read().parseBiggestInt, v: read().parseBiggestInt))

  sushi.sort(cmp)
  let
    x = sushi.mapIt(it.x)
    v = sushi.mapIt(it.v)
  var ans:int64 = 0
  ans = max(ans, solve(n, c, x, v))
  ans = max(ans, solve(n, c, x.mapIt(c - it).reversed, v.reversed))
  echo ans
main()
