import strutils, sequtils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m = read().parseInt
  var
    v = newSeqWith(n, read().parseInt)
    r = newSeqWith(m, read().parseInt)
  let A, B = read().parseInt

  const mo: int64 = 1000000007
  let
    vsum = v.sum
    rsum = r.sum
  var
    dp = newSeq[int64](vsum + 1)
    ep = newSeq[int64](rsum + 1)
  dp[0] = 1
  for i in 0..<n:
    for s in countdown(vsum, 0):
      if s + v[i] <= vsum:
        dp[s + v[i]] = (dp[s + v[i]] + dp[s]) mod mo
  ep[0] = 1
  for i in 0..<m:
    for s in countdown(rsum, 0):
      if s + r[i] <= rsum:
        ep[s + r[i]] = (ep[s + r[i]] + ep[s]) mod mo
  var c = newSeq[int64](ep.len + 1)
  for i in 1..ep.len:
    c[i] = c[i - 1]
    if i < ep.len:
      c[i] = (c[i] + ep[i]) mod mo
  # echo ep
  # echo c
  var ans: int64 = 0
  for s in 1..vsum:
    if dp[s] == 0:
      continue
    var
      i = (s + B - 1) div B
      j = min(rsum, s div A)
    if i <= j:
      # echo s, " ", i, " ", j
      let w = (c[j] - c[max(0, i - 1)] + mo) mod mo
      ans = (ans + dp[s] * w mod mo) mod mo
  echo ans
main()
