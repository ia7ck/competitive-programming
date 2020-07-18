import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip
  if s.count('R') > s.count('B'):
    echo "Yes"
  else:
    echo "No"

main()
