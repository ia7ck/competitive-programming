import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    s = read()
    tt = newSeqWith(n, read())
  var dp = newSeq[int64](s.len + 1)
  dp[0] = 1
  for i in 0..<s.len:
    for t in tt:
      if s[i..^1].startsWith(t):
        dp[i + t.len] = (dp[i + t.len] + dp[i]) mod 1000000007
  echo dp[s.len]
main()
