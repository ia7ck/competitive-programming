import strutils, sequtils, algorithm

# a <= b <= c
# (((a + b) / 2) / 2 +  c / 2)
#

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseInt)

  a.sort(cmp)
  var ans = (a[0] + a[1]).float64 / 2.0
  for i in 2..<n:
    ans = (ans + a[i].float64) / 2.0
  echo ans.formatFloat
main()
