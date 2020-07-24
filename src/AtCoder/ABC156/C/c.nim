import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
  var ans = int.high
  for p in 0..100:
    let cost = a.mapIt((it - p) * (it - p)).sum
    ans = min(ans, cost)
  echo ans
main()
