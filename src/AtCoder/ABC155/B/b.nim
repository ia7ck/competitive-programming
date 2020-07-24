import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  let ok = a.filterIt(it mod 2 == 0).allIt(it mod 3 == 0 or it mod 5 == 0)
  if ok:
    echo "APPROVED"
  else:
    echo "DENIED"
main()
