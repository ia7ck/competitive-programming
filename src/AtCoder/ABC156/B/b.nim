import strutils, sequtils

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var ans = 0
  while n > 0:
    n = n div k
    ans += 1
  echo ans
main()
