import strutils, sequtils, math

proc main() =
  let n = stdin.readLine.strip.parseInt
  let a = stdin.readLine.strip.split.map(parseInt)

  var
    s1 = 0
    s2 = a.sum
    ans = s2 - s1
  for i in 0..<(n - 1):
    s1 += a[i]
    s2 -= a[i]
    ans = min(ans, abs(s1 - s2))
  echo ans
main()
