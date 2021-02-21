import strutils, sequtils, algorithm, future

proc chmin(a: var int, b: int) =
  if a > b: a = b

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[a: int, b: int, i: int]
  var
    abi = newSeq[P]()
    cs = newSeqWith(m, newSeq[int]())
  for i in 0..<m:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    abi.add((a, b, i))
    cs[i] = stdin.readLine.strip.split.map(parseInt)

  abi.sort((x, y) => cmp(x.b, y.b))
  const inf: int = 1000000000 + 1
  var dp = newSeqWith(2, newSeqWith(1 shl n, inf))
  dp[0][0] = 0
  for j in 0..<m:
    let i = abi[j].i
    for bit in 0..<(1 shl n):
      chmin(dp[(j + 1) and 1][bit], dp[j and 1][bit])
      var y = 0
      for k in 0..<cs[i].len:
        y = y or (1 shl (cs[i][k] - 1))
      chmin(dp[(j + 1) and 1][bit or y], dp[j and 1][bit] + abi[j].a)
  var ans = dp[m and 1][(1 shl n) - 1]
  if ans == inf:
    ans = -1
  echo ans
main() #

#
