import strutils, sequtils

proc main() =
  var x, y: int
  (x, y) = stdin.readLine.strip.split.map(parseInt)
  if x == 1 and y == 1:
    echo 1000000
    return

  var ans = 0
  if x == 3:
    ans += 100000
  elif x == 2:
    ans += 200000
  elif x == 1:
    ans += 300000

  if y == 3:
    ans += 100000
  elif y == 2:
    ans += 200000
  elif y == 1:
    ans += 300000

  echo ans
main()
