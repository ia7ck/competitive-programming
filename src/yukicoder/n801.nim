import strutils, sequtils

const mo: int64 = 1000000000 + 7
proc add(a: var int64, b: int64) =
  a += b
  while a >= mo:
    a -= mo
proc sub(a: var int64, b: int64) =
  a -= b
  while a < 0:
    a += mo

proc main() =
  let
    nmk = stdin.readLine.strip.split.map(parseInt)
    (n, m, k) = (nmk[0], nmk[1], nmk[2])
    lr = (0..<m).mapIt(stdin.readLine.strip.split.map(parseInt))

  var dp = newSeq[int64](n + 1)
  dp[0] = 1
  for i in 0..<k:
    var
      nex = newSeq[int64](n + 1)
      acc = newSeq[int64](n + 1)
    for j in 0..<n:
      add(acc[j + 1], acc[j] + dp[j])
    for s in lr:
      var ad = acc[s[1]] - acc[s[0] - 1]
      if ad < 0:
        ad += mo
      add(nex[s[0] - 1], ad)
      sub(nex[s[1]], ad)
    for j in 0..<n:
      add(nex[j + 1], nex[j])
    swap(dp, nex)
  echo dp[n - 1]
main()
