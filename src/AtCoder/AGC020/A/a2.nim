import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseBiggestInt)
  var
    le: int64 = 2
    ri: int64 = 2
  for i in countdown(n - 1, 0):
    le = (le + a[i] - 1) div a[i] * a[i]
    ri = ri div a[i] * a[i] + a[i] - 1
  if le > ri: # ???
    echo -1
    return
  echo le, " ", ri
main()
