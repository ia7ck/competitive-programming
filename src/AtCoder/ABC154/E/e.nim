import strutils, sequtils
proc main() =
  let
    s = stdin.readLine.strip
    q = stdin.readLine.strip.parseInt
    n = s.len
  var dp = newSeqWith(n + 1, newSeqWith(2, newSeq[int](5)))
  dp[0][0][0] = 1
  for i in 0..<n:
    for less in 0..<2:
      for k in 0..3:
        let lim = if less == 1: 9 else: s[i].ord - '0'.ord
        for d in 0..lim:
          var nl = 0
          if less == 1 or d < lim:
            nl = 1
          var nk = k
          if d > 0:
            nk += 1
          if nk <= 3:
            dp[i + 1][nl][nk] += dp[i][less][k]
  echo dp[n][1][q] + dp[n][0][q]
main()
