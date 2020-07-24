import strutils, sequtils

proc main() =
  let abc = stdin.readLine.strip.split.map(parseInt)
  if abc[0] == abc[1] and abc[1] == abc[2]:
    echo "Yes"
  else:
    echo "No"

main()
