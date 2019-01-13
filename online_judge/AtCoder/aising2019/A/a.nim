import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.parseInt
    h = stdin.readLine.parseInt
    w = stdin.readLine.parseInt
    res = (n-h+1) * (n-w+1)
  echo res
main()
