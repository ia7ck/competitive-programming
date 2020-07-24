import strutils, sequtils

proc main() =
  var m1, d1: int
  (m1, d1) = stdin.readLine.strip.split.map(parseInt)
  var m2, d2: int
  (m2, d2) = stdin.readLine.strip.split.map(parseInt)

  if m1 == m2:
    echo 0
  else:
    echo 1
main()
