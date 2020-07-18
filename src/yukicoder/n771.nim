import strutils, sequtils

let
  n = stdin.readLine.parseInt
  ab = (0..<n).mapIt(stdin.readLine.split.map(parseInt))

var dp = newSeqWith(n, newSeqWith(1 shl n, 1000000000))
for i in 0..<n:
  dp[i][1 shl i] = 0

for bit in 0..<(1 shl n):
  for i in 0..<n:
    for j in 0..<n:
      let nb = bit xor (1 shl j)
      if nb>bit:
        dp[j][nb] = min(dp[j][nb],
        max(dp[i][bit], (ab[i][1]-ab[i][0])+ab[j][0]))
var mn = 1000000000
for i in 0..<n:
  mn = min(mn, dp[i][(1 shl n)-1])
echo mn
