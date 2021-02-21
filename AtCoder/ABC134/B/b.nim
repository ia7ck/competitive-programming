import strutils, sequtils

proc main() =
  var n, d: int
  (n, d) = stdin.readLine.strip.split.map(parseInt)

  var
    ans = 1
    a = d + 1
  while a + d < n:
    ans += 1
    a += d * 2 + 1
  echo ans
main()
