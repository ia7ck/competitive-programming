import strutils, sequtils, math

proc main() =
  var p, q, r: int
  (p, q, r) = stdin.readLine.strip.split.map(parseInt)

  echo (p + q + r - max(p, q, r))

main()
