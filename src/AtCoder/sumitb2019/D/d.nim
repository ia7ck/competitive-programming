import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip
  var
    dp = newSeqWith(4, newSeqWith(10, newSeqWith(10, newSeqWith(10, false))))
    nxt = newSeqWith(4, newSeqWith(10, newSeqWith(10, newSeqWith(10, false))))
  dp[0][0][0][0] = true
  for i in 0..<n:
    for a in 0..<4:
      for b in 0..<10:
        for c in 0..<10:
          for d in 0..<10:
            nxt[a][b][c][d] = dp[a][b][c][d]
    let x = s[i].ord - '0'.ord
    for a in 0..<3:
      for b in 0..<10:
        for c in 0..<10:
          for d in 0..<10:
            if dp[a][b][c][d]:
              nxt[a + 1][c][d][x] = true
    swap(dp, nxt)
  var ans = 0
  for b in 0..<10:
    for c in 0..<10:
      for d in 0..<10:
        if dp[3][b][c][d]:
          ans += 1
  echo ans
main()

