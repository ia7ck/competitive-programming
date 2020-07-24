import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
  var s = 0
  for i in 0..<n:
    for j in 0..<i:
      s += a[i] * a[j]
  echo s
main()
