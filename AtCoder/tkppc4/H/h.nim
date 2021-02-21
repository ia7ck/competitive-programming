import strutils, sequtils, algorithm

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var a = stdin.readLine.strip.split.map(parseInt)

  a.sort(system.cmp)
  var ans: float64 = 0
  for i in 0..<k:
    ans += (a[i] + a[n - k + i]).float64 / a[n - k + i].float64
  echo ans.formatFloat

main()
