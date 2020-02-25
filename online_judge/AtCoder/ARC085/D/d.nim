import strutils, sequtils

proc main() =
  var n, z, w: int
  (n, z, w) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  if n == 1:
    echo abs(w - a[n - 1])
    return
  echo max(abs(w - a[n - 1]), abs(a[n - 2] - a[n - 1]))

main()
