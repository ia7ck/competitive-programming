import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.parseInt
    ab = stdin.readLine.split.map(parseInt)
    (a, b) = (ab[0], ab[1])
    ps = stdin.readLine.split.map(parseInt)
  var x, y, z: int
  for p in ps:
    if p<=a:
      x+=1
    elif p<=b:
      y+=1
    else:
      z+=1
  echo min(x, y, z)
main()
