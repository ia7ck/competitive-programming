import strutils, sequtils, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

# n 以下で条件を満たす数の個数
proc count(n: int64): int =
  let s = $n
  var dp = newSeqWith(s.len + 1, newSeqWith(2, newSeqWith(2, newSeqWith(10, 0))))
  dp[0][0][0][0] = 1 # ???
  for i in 0..<s.len:
    for j in 0..<2:
      for z in 0..<2:
        for k in 0..<10:
          let lim = if j == 1: 9 else: s[i].ord - '0'.ord
          for d in 0..lim:
            var nj = j
            if d < lim:
              nj = 1
            var nz = z
            if d > 0:
              nz = 1
            if z == 0 or abs(k - d) <= 1:
              dp[i + 1][nj][nz][d] += dp[i][j][z][k]
  return dp[s.len][0][1].sum + dp[s.len][1][1].sum

proc main() =
  let k = read().parseInt

  var
    ok: int64 = 4 * 1000000000
    ng: int64 = 0
  while ok - ng > 1:
    let md = (ok + ng) div 2
    if count(md) >= k:
      ok = md
    else:
      ng = md
  echo ok
main()
