import strutils, sequtils

proc main() =
  var n, r: int
  (n, r) = stdin.readLine.strip.split.map(parseInt)
  if n >= 10:
    echo r
  else:
    echo r + 100 * (10 - n)

main()
