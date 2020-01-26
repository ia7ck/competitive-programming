import strutils, sequtils
proc main() =
  var h, a: int
  (h, a) = stdin.readLine.strip.split.map(parseInt)

  echo ((h + a - 1) div a)
main()
