import strutils, sequtils, algorithm
proc main() =
  let
    n = stdin.readLine.parseInt
    a = (0..<n).mapIt(stdin.readLine.parseInt)
  echo a.foldl(a+b)-a.max div 2
main()
