import strutils, sequtils

proc main() =
  var a, b, c: int
  (a, b, c) = stdin.readLine.strip.split.map(parseInt)
  if a == b and b == c:
    echo "No"
    return
  if a == b or b == c or c == a:
    echo "Yes"
  else:
    echo "No"
main()
