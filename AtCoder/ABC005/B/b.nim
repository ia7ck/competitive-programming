import strutils, sequtils

let
  n = stdin.readLine.parseInt
  t = (0..<n).mapIt(stdin.readLine.parseInt)
echo t.min
