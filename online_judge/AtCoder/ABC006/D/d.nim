import strutils, sequtils, algorithm

let
  n = stdin.readLine.parseInt
  ai = (0..<n).mapIt(stdin.readLine.parseInt)

var dp = newSeqWith(n+1, 100000)
dp[0] = 0
for a in ai:
  let j = dp.lowerBound(a)
  if j-1>=0:
    dp[j] = a
echo n-dp.lowerBound(100000)+1
