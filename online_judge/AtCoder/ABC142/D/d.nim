import strutils, sequtils, math

proc main() =
  var a, b: int64
  (a, b) = stdin.readLine.strip.split.map(parseBiggestInt)

  var
    g = gcd(a, b)
    i: int64 = 2
    ans = 1
  while i * i <= a:
    if g mod i == 0:
      ans += 1
      while g mod i == 0:
        g = g div i
    i += 1
  if g > 1:
    ans += 1
  echo ans
main()
