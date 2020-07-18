import strutils, sequtils, math

const mo:int64 = 1000000000 + 7
proc add(a: var int64, b: int64) =
  a = (a mod mo+ b mod mo) mod mo

proc main() =
  let a = stdin.readLine.strip
  var dp = newSeq[int64](2)
  dp[0] = 0
  dp[1] = 1
  for c in a:
    var nxt = newSeq[int64](2)
    if c == '0':
      add(nxt[0], dp[0] * 3)
      add(nxt[1], dp[1])
    else:
      add(nxt[0], dp[0] * 3)
      add(nxt[0], dp[1])
      add(nxt[1], dp[1] * 2)
    swap(dp, nxt)
  echo dp.sum mod mo
main()
