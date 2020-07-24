import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseBiggestInt
  if n <= 2:
    echo 0
    return
  if n == 3:
    echo 2
    return
  var sq: int64 = 1
  while (sq + 1) * (sq + 1) <= n:
    sq += 1
  var ans: int64 = 0
  for m in 1..sq:
    if (n div m) == (n mod m):
      ans += m
    if (n - m) mod m == 0:
      let ad = (n - m) div m
      if (n div ad) == (n mod ad):
        ans += ad
  echo ans
main()
