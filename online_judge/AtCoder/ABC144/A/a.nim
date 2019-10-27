import strutils, sequtils

proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)

  if a <= 9 and b <= 9:
    echo a * b
  else:
    echo -1
main()
