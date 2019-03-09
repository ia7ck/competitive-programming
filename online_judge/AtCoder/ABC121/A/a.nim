import strutils, sequtils

proc main() =
  let
    hw = stdin.readLine.strip.split.map(parseInt)
    rc = stdin.readLine.strip.split.map(parseInt)
  echo((hw[0] - rc[0]) * (hw[1] - rc[1]))

main()
