import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  echo s[0]
main()
