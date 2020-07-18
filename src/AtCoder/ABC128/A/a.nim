import strutils, sequtils

proc main() =
  var a, p: int
  (a, p) = stdin.readLine.strip.split.map(parseInt)
  p += a * 3
  echo p div 2

main()
