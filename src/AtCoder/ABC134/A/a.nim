import strutils, sequtils

proc main() =
  let r = stdin.readLine.strip.parseInt
  echo 3 * r * r
main()
