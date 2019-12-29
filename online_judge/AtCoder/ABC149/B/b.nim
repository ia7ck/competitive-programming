import strutils, sequtils
proc main() =
  var a, b, k: int64
  (a, b, k) = stdin.readLine.strip.split.map(parseBiggestInt)

  if k <= a:
    echo a - k, " ", b
  else:
    echo 0, " ", max(0, b - (k - a))
main()
