import strutils, sequtils

proc f(x: int): int =
  var y = 1
  while x div y > 0:
    result = (x div y) mod 10
    y = y * 10

proc main() =
  let n = stdin.readLine.strip.parseInt

  var freq = newSeqWith(10, newSeq[int64](10))
  for i in 1..n:
    let
      t = f(i)
      b = i mod 10
    freq[t][b] += 1
  var ans: int64 = 0
  for i in 1..n:
    let
      t = f(i)
      b = i mod 10
    ans += freq[b][t]
  echo ans
main()
