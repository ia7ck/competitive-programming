import strutils, sequtils, algorithm, intsets

proc compress(a: seq[int]): seq[int] =
  let
    n = a.len
    b = (0..<n).mapIt((v: a[it], i: it)).sorted(cmp)
  var
    s = initIntSet()
    card = 0
  result = newSeq[int](n)
  for it in b:
    if not s.contains(it.v):
      s.incl(it.v)
      card += 1
    result[it.i] = card

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = newSeqWith(n, stdin.readLine.strip.parseInt)
  a.reverse
  let b = compress(a)
  var dp = newSeq[int](n + 1)
  fill(dp, int.high)
  dp[0] = 0
  for it in b:
    let j = dp.lowerBound(it + 1)
    dp[j] = min(dp[j], it)
  echo dp.filterIt(it < int64.high).len - 1
main()
