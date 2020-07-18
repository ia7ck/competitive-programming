import strutils, sequtils
proc main() =
  var k, x: int
  (k, x) = stdin.readLine.strip.split.map(parseInt)
  if k * 500 >= x:
    echo "Yes"
  else:
    echo "No"
main()
