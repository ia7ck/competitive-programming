import strutils, sequtils, algorithm

proc main() =
  var L, R: int64
  (L, R) = stdin.readLine.strip.split.map(parseBiggestInt)

  if R - L >= 2019:
    echo 0
    return
  var ans: int64 = 2019
  for i in L..R:
    for j in (i + 1)..R:
      ans = min(ans, (i * j) mod 2019)
  echo ans
main()
