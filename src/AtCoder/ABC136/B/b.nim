import strutils, sequtils

proc f(x: int): int =
  var
    y = x
    k = 0
  while y > 0:
    y = y div 10
    k += 1
  return k

proc main() =
  let n = stdin.readLine.strip.parseInt

  var ans = 0
  for i in 1..n:
    if f(i) mod 2 == 1:
      ans += 1
  echo ans
main()
