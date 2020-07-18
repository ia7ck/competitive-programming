import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.parseInt
    a = (0..<n).mapIt(stdin.readLine.parseInt)
  if a.anyIt(it mod 2 == 1):
    echo "first"
  else:
    echo "second"
main()
