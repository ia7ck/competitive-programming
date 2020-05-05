import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseInt)
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let u, v = read().parseInt
    g[u - 1].add(v - 1)
    g[v - 1].add(u - 1)
  const inf = int.high
  var dp = newSeqWith(n + 1, inf)
  dp[0] = 0
  dp[1] = a[0]
  var ans = newSeq[int](n)
  ans[0] = 1
  proc dfs(i, p: int)=
    for j in g[i]:
      if j != p:
        let
          k = dp.lowerBound(a[j])
          val = dp[k]
        # echo j, " ", dp
        dp[k] = min(dp[k], a[j])
        # echo j, " ", dp
        ans[j] = dp.lowerBound(inf) - 1
        dfs(j, i)
        dp[k] = val
  dfs(0, -1)
  echo ans.join("\n")
main()
