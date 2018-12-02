import strutils, sequtils

proc main() =
  let x = stdin.readLine.parseInt
  if x==3 or x==5 or x==7:
    echo "YES"
  else:
    echo "NO"

main()
