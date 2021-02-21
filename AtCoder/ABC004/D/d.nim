import strutils, sequtils

proc chmin(l: var int, r: int) =
  if l>r: l = r

let
  rgb = stdin.readLine.split.map(parseInt)
  (r, g, b) = (rgb[0], rgb[1], rgb[2])
  n = 1000
  (pos_r, pos_g, pos_b) = (400, 500, 600)

var dp = newSeqWith(n, newSeqWith(r+g+b+1, 1_000_000_000))
dp[0][0] = 0
for i in 1..<n:
  for j in 0..(r+g+b):
    chmin(dp[i][j], dp[i-1][j])
    if j>0:
      if j<=r:
        chmin(dp[i][j], dp[i-1][j-1]+abs(pos_r-i))
      elif j<=r+g:
        chmin(dp[i][j], dp[i-1][j-1]+abs(pos_g-i))
      elif j<=r+g+b:
        chmin(dp[i][j], dp[i-1][j-1]+abs(pos_b-i))
var mn = 1_000_000_000
for i in 0..<n: chmin(mn, dp[i][r+g+b])
echo mn
