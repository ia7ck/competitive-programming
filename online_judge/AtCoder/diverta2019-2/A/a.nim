import strutils, sequtils

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)

  if k == 1:
    echo 0
  else:
    echo n - k
main()
