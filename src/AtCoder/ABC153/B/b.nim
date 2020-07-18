import strutils, sequtils, math
proc main() =
  var h, n: int
  (h, n) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  if h <= a.sum:
    echo "Yes"
  else:
    echo "No"
main()
