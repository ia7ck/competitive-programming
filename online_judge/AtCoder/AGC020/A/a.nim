import strutils, sequtils

proc main() =
  let
    nab = stdin.readLine.strip.split.map(parseInt)
  if (nab[2] - nab[1]) mod 2 == 0:
    echo "Alice"
  else:
    echo "Borys"
main()
