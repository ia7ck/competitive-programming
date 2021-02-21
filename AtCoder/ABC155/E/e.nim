import strutils, sequtils, algorithm

proc chmin(a: var int64, b: int64) =
  a = min(a, b)

proc main() =
  let a = stdin.readLine.strip.mapIt(it.ord - '0'.ord)

  let cost = @[0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1]
  var dp = newSeq[int64](2)
  dp[0] = cost[a[0]]
  dp[1] = cost[a[0] + 1]
  for it in a[1..^1]:
    var nxt = newSeq[int64](2)
    fill(nxt, 1000000000000000000)
    chmin(nxt[0], dp[0] + cost[it]) # ちょうど払う
    chmin(nxt[0], dp[1] + cost[10 - it]) # お釣り
    chmin(nxt[1], dp[0] + cost[it + 1]) # 多めに払う, 後でお釣りもらう
    chmin(nxt[1], dp[1] + cost[9 - it]) # 少なめにお釣りもらう
    swap(dp, nxt)
  echo dp[0]
main()
