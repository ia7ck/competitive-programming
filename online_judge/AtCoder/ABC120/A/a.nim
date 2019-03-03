import strutils, sequtils

proc main() =
  let
    abc = stdin.readLine.strip.split.map(parseInt)
    (a, b, c) = (abc[0], abc[1], abc[2])
  echo min(c, b div a)

main()
