import strutils, sequtils, math, algorithm

const mo = 1000000000 + 7
proc add(x: var int, y: int) =
  x += y
  while x >= mo:
    x -= mo

proc main() =
  let
    nk = stdin.readLine.strip.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
  var
    a = (0..<n).mapIt(stdin.readLine.strip.parseInt)
    s = a.sum

  a.sort(system.cmp)
  var
    dp = newSeqWith(2, newSeq[int](max(k, s) + 1))
    t = 0
  dp[0][0] = 1
  for i in 0..<n:
    let
      frm = i and 1
      to = frm xor 1
    for j in 0..t:
      dp[to][j] = 0
    for j in 0..t:
      add(dp[to][j], dp[frm][j])
      add(dp[to][j xor a[i]], dp[frm][j])
    t = t or a[i]
  echo dp[n and 1][k]

main()
