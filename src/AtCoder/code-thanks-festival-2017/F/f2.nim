import strutils, sequtils, math, algorithm

const mo: int64 = 1000000000 + 7
proc add(x: var int64, y: int64) =
  x += y
  while x >= mo:
    x -= mo
proc modpow(a, b: int64): int64 =
  if b == 0:
    return 1
  elif b == 1:
    return a
  elif b mod 2 == 0:
    return modpow(a * a mod mo, b div 2)
  else:
    return a * modpow(a, b - 1) mod mo

proc main() =
  let
    nk = stdin.readLine.strip.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
  var
    a = (0..<n).mapIt(stdin.readLine.strip.parseInt)
    s = a.sum

  a.sort(system.cmp)
  type T = object
    val: int
    freq: int
  var b = newSeq[T]()
  b.add(T(val: a[0], freq: 1))
  for i in 1..<n:
    if a[i - 1] == a[i]:
      b[b.len - 1].freq += 1
    else:
      b.add(T(val: a[i], freq: 1))
  var dp = newSeqWith(2, newSeq[int64](max(k, s) + 1))
  dp[0][0] = 1
  let
    m = b.len
    inv2 = modpow(2, mo - 2)
  for i in 0..<m:
    let
      frm = i and 1
      to = frm xor 1
      p = modpow(2, b[i].freq) * inv2 mod mo
    for j in 0..max(k, s):
      dp[to][j] = 0
    for j in 0..max(k, s):
      add(dp[to][j], dp[frm][j] * p mod mo)
      if (j xor b[i].val) <= max(k, s):
        add(dp[to][j xor b[i].val], dp[frm][j] * p mod mo)
  echo dp[m and 1][k]

main()
