import strutils, sequtils

let MOD = 1_000_000_000+7
proc madd(a: var int64, b: int64) =
  a = (a+b) mod MOD

proc main() =
  let
    n = stdin.readLine
    m = stdin.readLine.parseInt
  var dp = newSeqWith(n.len+1, newSeqWith(2, newSeqWith(m, 0'i64)))
  dp[0][0][0] = 1
  for i in 0..<n.len:
    for less in 0..<2:
      for sum in 0..<m:
        let digit = if less==0: n[i].int - ('0').int else: 9
        for d in 0..digit:
          let nl = if d<digit: 1 else: less
          madd(dp[i+1][nl][(sum+d) mod m], dp[i][less][sum])
  let res = (dp[n.len][0][0]+dp[n.len][1][0]-1+MOD) mod MOD
  echo res
main()
