import strutils, sequtils

proc main() =
  let
    nd = stdin.readLine.strip.split.map(parseInt)
    (n, d) = (nd[0], nd[1])

  var freq = newSeq[int64](n * n * 2 + 1)
  for x in 1..n:
    for y in 1..n:
      freq[x * x + y * y] += 1
  var ans: int64 = 0
  for z in 1..n:
    for w in 1..n:
      let v = w * w + d - z * z
      if v >= 0 and v < freq.len:
        ans += freq[v]
  echo ans
main()
