import strutils, sequtils

proc main() =
  let
    nm = stdin.readLine.strip.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    a = stdin.readLine.strip.split.map(parseInt)

  var
    freq = newSeq[int](n)
    cost = newSeqWith(n, newSeq[int64](n)) # iが提供されたときにjを待ってる人がうける不満度の総和
  for idx in countdown(m-1, 0):
    for i in 0..<n:
      if i != a[idx]:
        cost[i][a[idx]] += freq[i]
    freq[a[idx]] += 1
  var dp = newSeqWith(1 shl n, 1_000_000_000_000_000_000)
  dp[0] = 0
  for bits in 0..<(1 shl n):
    for i in 0..<n:
      if (bits and (1 shl i)) == 0:
        var c: int64 = 0
        for j in 0..<n:
          if (bits and (1 shl j)) == 0:
            c += cost[i][j]
        dp[bits xor (1 shl i)] = min(
          dp[bits xor (1 shl i)],
          dp[bits] + c
        )
  echo dp[(1 shl n) - 1]
main()
