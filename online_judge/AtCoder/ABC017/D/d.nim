import strutils, sequtils
proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    a = @[0] & (0..<n).mapIt(stdin.readLine.parseInt)
    MOD = 1_000_000_000+7
  var
    i = 0
    s = 1
    dp = newSeq[int](n+1)
    last = newSeqWith(m+1, -1)
  dp[0] = 1
  for j in 1..n:
    while i<last[a[j]]:
      s = (s-dp[i]+MOD) mod MOD
      i+=1
    last[a[j]] = j
    dp[j] = s
    s = (s+dp[j]) mod MOD
  echo dp[n]

# * 1 2 1 2 2

# 1 - - - - -
# 1 1 - - - - # [0, 1)
# 1 1 2 - - - # [0, 2)
# 1 1 2 3 - - # [1, 3)
# 1 1 2 3 5 - # [2, 4)
# 1 1 2 3 5 5 # [4, 5)

main()
