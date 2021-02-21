import strutils, sequtils
proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)

  let
    s = ($a).repeat(b)
    t = ($b).repeat(a)
  echo min(s, t)

main()
