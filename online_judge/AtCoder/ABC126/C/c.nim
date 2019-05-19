import strutils, sequtils

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var ans: float64 = 0.0
  for i in 1..n:
    var
      tmp = i
      p: float64 = 1.0 / n.float64
    while tmp < k:
      tmp *= 2
      p *= 0.5
    ans += p
  echo ans.formatFloat
main()
