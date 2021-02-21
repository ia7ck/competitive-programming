import strutils, sequtils

const mo: int64 = 998244353
proc powmod(a, x: int64): int64 =
  if x == 0:
    return 1
  elif x == 1:
    return a
  elif x mod 2 == 0:
    return powmod(a * a mod mo, x div 2)
  else:
    return a * powmod(a, x - 1) mod mo


proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  if a[0] != 0:
    echo 0
    return
  if a[1..^1].filterIt(it == 0).len > 0:
    echo 0
    return
  var freq = newSeq[int](n)
  for it in a:
    freq[it] += 1
  var r: int
  for i in 0..<n:
    if freq[i] > 0:
      r = i
  if freq[1..r].filterIt(it == 0).len > 0:
    echo 0
    return
  var ans: int64 = 1
  for i in 1..r:
    ans = ans * powmod(freq[i - 1], freq[i]) mod mo
  echo ans


main()
