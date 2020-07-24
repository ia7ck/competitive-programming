import strutils, sequtils

proc f(dp: seq[seq[int]], s, t: string, i, j: int): string =
  if i==0 and j==0:
    result = ""
  else:
    if i-1>=0 and dp[i-1][j]==dp[i][j]:
      result = f(dp, s, t, i-1, j)
    elif j-1>=0 and dp[i][j-1]==dp[i][j]:
      result = f(dp, s, t, i, j-1)
    elif i-1>=0 and j-1>=0 and s[i-1]==t[j-1]:
      result = f(dp, s, t, i-1, j-1) & s[i-1]
    else:
      doAssert(false)

proc chmax(x: var int, y: int) =
  if x<y:
    x = y

proc main() =
  let
    s = stdin.readLine
    t = stdin.readLine
    (n, m) = (s.len, t.len)
  var dp = newSeqWith(n+1, newSeqWith(m+1, 0))
  for i in 0..n:
    for j in 0..m:
      if i+1<=n:
        chmax(dp[i+1][j], dp[i][j])
      if j+1<=m:
        chmax(dp[i][j+1], dp[i][j])
      if i+1<=n and j+1<=m and s[i]==t[j]:
        chmax(dp[i+1][j+1], dp[i][j]+1)
  echo f(dp, s, t, n, m)

main()
