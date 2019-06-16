import strutils, sequtils

proc main() =
  var n, x: int
  (n, x) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  var
    cnt = 1
    pos = 0
  for it in a:
    pos += it
    if pos <= x:
      cnt += 1
  echo cnt
main()
