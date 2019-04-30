import strutils, sequtils, algorithm

proc main() =
  var n, x, y: int
  (n, x, y) = stdin.readLine.strip.split.map(parseInt)
  var a = stdin.readLine.strip.split.map(parseInt)

  a.sort(system.cmp)
  a.reverse
  for i in 0..<n:
    if i mod 2 == 0:
      x += a[i]
    else:
      y += a[i]
  if x > y:
    echo "Takahashi"
  elif x < y:
    echo "Aoki"
  else:
    echo "Draw"

main()
