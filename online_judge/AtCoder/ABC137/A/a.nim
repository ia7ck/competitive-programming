import strutils, sequtils, math

proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)
  var ans = a + b
  ans = max(ans, a - b)
  ans = max(ans, a * b)
  echo ans

main()
