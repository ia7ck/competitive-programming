import strutils, sequtils
proc main() =
  let st = stdin.readLine.strip.split
  echo (st[1] & st[0])
main()
