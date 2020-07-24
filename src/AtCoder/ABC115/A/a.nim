import strutils, sequtils
proc main() =
  let d = stdin.readLine.parseInt
  var s = "Christmas"
  for _ in 0..<(25-d):
    s &= " Eve"
  echo s
main()
