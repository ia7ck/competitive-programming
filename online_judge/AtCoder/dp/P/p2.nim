import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  const mo: int64 = 1000000000 + 7
  var dp = newSeqWith(n, newSeqWith(2, -1.int64))
  proc dfs(i, p, c: int): int64 =
    if dp[i][c] >= 0:
      return dp[i][c]
    dp[i][c] = 1
    for j in g[i]:
      if j == p:
        continue
      var a = dfs(j, i, 0)
      if c == 0:
        a = (a + dfs(j, i, 1)) mod mo
      dp[i][c] = dp[i][c] * a mod mo
    return dp[i][c]
  let ans = (dfs(0, -1, 0) + dfs(0, -1, 1)) mod mo
  echo ans
main()
