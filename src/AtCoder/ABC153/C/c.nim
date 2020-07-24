import strutils, sequtils, algorithm, math
proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var a = stdin.readLine.strip.split.map(parseBiggestInt)

  if k >= n:
    echo 0
    return
  a.sort(system.cmp)
  a.reverse
  echo a[k..^1].sum
main()
