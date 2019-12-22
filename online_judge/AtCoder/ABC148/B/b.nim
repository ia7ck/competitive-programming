import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    st = stdin.readLine.strip.split
  var u = ""
  for i in 0..<n:
    u.add(st[0][i])
    u.add(st[1][i])
  echo u
main()
