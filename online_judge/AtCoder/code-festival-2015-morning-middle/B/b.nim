import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    nn = read().parseInt
    s = read()

  proc chmax(x: var int, y: int) =
    if x < y: x = y

  proc lcs(a, b: string): int =
    let
      n = a.len
      m = b.len
    var dp = newSeqWith(n + 1, newSeqWith(m + 1, 0))
    for i in 0..n:
      for j in 0..m:
        if i < n:
          chmax(dp[i + 1][j], dp[i][j])
        if j < m:
          chmax(dp[i][j + 1], dp[i][j])
        if i < n and j < m:
          if a[i] == b[j]:
            chmax(dp[i + 1][j + 1], dp[i][j] + 1)
          else:
            chmax(dp[i + 1][j + 1], dp[i][j])
    return dp[n][m]
  var ans = nn
  for i in 1..<nn:
    let
      a = s[0..<i]
      b = s[i..^1]
    let len = lcs(a, b)
    # echo (a, b, len)
    ans = min(ans, a.len - len + b.len - len)
  echo ans
main()
