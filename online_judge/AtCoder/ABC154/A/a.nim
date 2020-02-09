import strutils, sequtils
proc main() =
  var s, t: string
  (s, t) = stdin.readLine.strip.split
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)
  let u = stdin.readLine.strip

  if s == u:
    echo a - 1, " ", b
  else:
    echo a, " ", b - 1
main()
