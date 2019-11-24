import strutils, sequtils

proc d(n: int64): int =
  var
    ret = 1
    p = 10
  while n div p > 0:
    ret += 1
    p *= 10
  return ret

proc main() =
  var a, b, x: int64
  (a, b, x) = stdin.readLine.strip.split.map(parseBiggestInt)

  var
    ok = 0
    ng = 1000000001
  while ng - ok > 1:
    let m = (ng + ok) div 2
    if a * m + b * d(m) <= x:
      ok = m
    else:
      ng = m
  echo ok

main()
