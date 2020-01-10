import strutils, sequtils, math
proc main() =
  var n, mm: int
  (n, mm) = stdin.readLine.strip.split.map(parseInt)
  let m = mm.int64
  let a = stdin.readLine.strip.split.map(parseBiggestInt)

  var p: int64 = 1
  for it in a:
    p = lcm(p, it div 2)
    if p > m:
      echo 0
      return
  for it in a:
    if (p div (it div 2)) mod 2 == 0:
      echo 0
      return
  var ans = m div p - 1
  ans = ans div 2
  ans += 1
  echo ans
main()
