import strutils, sequtils

proc main() =
  var r, d, x: int64
  (r, d, x) = stdin.readLine.strip.split.map(parseBiggestInt)
  for i in 1..10:
    x = r * x - d
    echo x
main()
