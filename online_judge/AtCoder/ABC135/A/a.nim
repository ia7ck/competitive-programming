import strutils, sequtils

proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)

  if (a + b) mod 2 == 0:
    echo ((a + b) div 2)
  else:
    echo "IMPOSSIBLE"
main()
