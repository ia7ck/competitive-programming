import strutils, sequtils, math
proc main() =
  var n, k, m: int
  (n, k, m) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  let r = max(0, n * m - a.sum)
  if r > k:
    echo -1
  else:
    echo r
main()
