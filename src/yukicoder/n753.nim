import strutils, sequtils, algorithm

proc popcnt(n: int): int =
  result = (n and 0x55555555) + (n shr 1 and 0x55555555)
  result = (result and 0x33333333) + (result shr 2 and 0x33333333)
  result = (result and 0x0f0f0f0f) + (result shr 4 and 0x0f0f0f0f)
  result = (result and 0x00ff00ff) + (result shr 8 and 0x00ff00ff)
  result = (result and 0x0000ffff) + (result shr 16 and 0x0000ffff)

proc ispow2(n: int): bool =
  result = false
  for i in 0..4:
    if n==(1 shl i):
      result = true

proc main() =
  let
    n = 16
    a = (0..<n).mapIt(stdin.readLine.split.map(parseInt))

  var dp = newSeqWith(n, newSeq[int64](1 shl n))
  for i in 0..<n: dp[i][1 shl i] = 1
  for bit in 0..<(1 shl n):
    let par_size = popcnt(bit)
    if ispow2(par_size):
      var sub = bit
      while sub>0:
        if popcnt(sub)==(par_size div 2):
          let sub2 = bit xor sub
          for i in 0..<n:
            if ((sub shr i) and 1)==1:
              for j in 0..<n:
                if ((sub2 shr j) and 1)==1:
                  let w = if i<j: a[i][j] else: -a[j][i]
                  if w == 1:
                    dp[i][bit]+=dp[i][sub]*dp[j][sub2]
                  elif w == -1:
                    dp[j][bit]+=dp[i][sub]*dp[j][sub2]

        sub = (sub-1) and bit
  for i in 0..<n: echo dp[i][(1 shl n)-1]

main()
