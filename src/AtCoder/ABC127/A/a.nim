import strutils, sequtils

proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)
  if a <= 5:
    echo 0
  elif a <= 12:
    echo b div 2
  else:
    echo b
main()
