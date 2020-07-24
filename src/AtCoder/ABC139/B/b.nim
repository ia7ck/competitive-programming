import strutils, sequtils

proc main() =
  var a, b: int
  (a, b) = stdin.readLine.strip.split.map(parseInt)

  var
    cur = 1
    need = 0
  while cur < b:
    cur = cur - 1 + a
    need += 1
  echo need
main()
