import strutils, sequtils

proc main() =
  let
    nab = stdin.readLine.split.map(parseInt)
    (n, a, b) = (nab[0], nab[1], nab[2])
  echo min(a, b), " ", max(0, (a+b)-n)

main()
