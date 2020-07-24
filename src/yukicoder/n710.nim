import strutils, sequtils, algorithm

var
  n=stdin.readLine.parseInt
  dp=newSeq[int](1000*100+1)
  # dp[t]:=sum_{i in I}(A_i)=tを満たすIに対して、max_{I}(\sum_{i in I}(B_i))
  sb=0

fill(dp, -1)
dp[0]=0
for _ in 0..<n:
  var
    args=stdin.readLine.split.map(parseInt)
    (a, b)=(args[0], args[1])
  sb+=b
  for t in countdown(1000*100, 0):
    if dp[t]<0:
      continue
    if t+a<dp.len:
      dp[t+a]=max(dp[t+a], dp[t]+b)

var mn=1_000_000_000
for t in 0..1000*100:
  if dp[t]<0:
    continue
  mn=min(mn, max(t, sb-dp[t]))

echo mn