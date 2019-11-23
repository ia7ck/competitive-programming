import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseBiggestInt)
    s = a.sum

  var t: int64 = 0
  for j, it in a:
    t += it
    let s2 = s div 2
    if t == s2:
      echo 0
      return
    if t > s2:
      var ans = t - (s - t)
      ans = min(ans, s - t + it - (t - it))
      echo ans
      return
  doAssert(false)

main()
