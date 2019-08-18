import strutils, sequtils

proc main() =
  let
    a = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip
  if a >= 3200:
    echo s
  else:
    echo "red"
main()
