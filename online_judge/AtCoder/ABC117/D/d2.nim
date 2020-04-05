import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    k = read().parseBiggestInt.toBin(44)
    a = newSeqWith(n, read().parseBiggestInt)
  var dp = newSeqWith(k.len + 1, newSeqWith(2, -1.int64))
  dp[0][0] = 0
  for i in 0..<k.len:
    for j in 0..<2:
      if dp[i][j] < 0:
        continue
      let lim = if j == 1: 1 else: k[i].ord - '0'.ord
      for d in 0..lim:
        let nj = if d < lim: 1 else: j
        var cnt = 0
        for it in a:
          cnt += (it shr (k.len - i - 1)) and 1
        if d == 0:
          dp[i + 1][nj] = max(dp[i + 1][nj], dp[i][j] + cnt * (1 shl (k.len - i - 1)))
        else:
          dp[i + 1][nj] = max(dp[i + 1][nj], dp[i][j] + (n - cnt) * (1 shl (k.len - i - 1)))
  echo max(dp[k.len][0], dp[k.len][1])
main()
