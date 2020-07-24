import strutils, sequtils
proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  if n == m:
    echo "Yes"
  else:
    echo "No"
main()
