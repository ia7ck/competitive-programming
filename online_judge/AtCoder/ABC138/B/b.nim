import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  var s: float64 = 0
  for it in a:
    s += 1.0 / it.float64
  let ans = 1.0 / s
  echo ans.formatFloat
main()
