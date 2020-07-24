import strutils, sequtils

proc main() =
  var w, h, x, y: int
  (w, h, x, y) = stdin.readLine.strip.split.map(parseInt)

  let dup = if x * 2 == w and y * 2 == h: 1 else: 0
  echo((w * h).float64 / 2.0, " ", dup)
main()
