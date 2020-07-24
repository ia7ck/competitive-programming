import strutils, sequtils

proc main() =
  var x, a: int
  (x, a) = stdin.readLine.strip.split.map(parseInt)

  if x < a:
    echo 0
  else:
    echo 10
main()
