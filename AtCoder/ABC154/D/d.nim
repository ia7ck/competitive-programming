import strutils, sequtils, algorithm, math
proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseFloat)

  var
    s = a[0..<k].mapIt((it + 1) / 2.0).sum
    ans = s
  for i in k..<n:
    s -= (a[i - k] + 1) / 2.0
    s += (a[i] + 1) / 2.0
    ans = max(ans, s)
  echo ans.formatFloat
main()
