import strutils, sequtils

proc pow10(x: int64): int64 =
  var ten: int64 = 1
  while x div ten > 0:
    ten *= 10
  return ten

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    args = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))
  var
    mi = newSeq[int](n)
    ai = newSeqWith(n, newSeq[int64]())
  for i in 0..<n:
    mi[i] = args[i][0]
    for j in 1..mi[i]:
      ai[i].add(args[i][j])
  var
    dp = newSeq[int64]()
    mo: int64 = 1000000000 + 7
    prod: int64 = 1
  dp.add(0)
  for i in 0..<n:
    var
      nxt = newSeq[int64](mi[i])
      s: int64 = 0
      k: int64 = 0
    for it in dp:
      s = (s + it) mod mo
      k += 1
    prod = (prod * k) mod mo
    for j in 0..<mi[i]:
      let prev = s * (pow10(ai[i][j]) mod mo) mod mo
      nxt[j] = (prev + (ai[i][j] * prod mod mo)) mod mo
    dp.swap(nxt)
  var ans: int64 = 0
  for it in dp:
    ans = (ans + it) mod mo
  echo ans
main()
