import strutils, sequtils
proc main() =
  let xy = stdin.readLine.split.map(parseInt)
  var (x, y) = (xy.max, xy.min)

  let d = x-y
  x-=d div 2 * 3
  y-=d div 2
  if x>=0 and y>=0:
    assert x==y
    echo if x mod 4 == 0: "Yes" else: "No"
  else:
    echo "No"

main()
