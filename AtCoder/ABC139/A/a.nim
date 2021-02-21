import strutils, sequtils

proc main() =
  let
    s = stdin.readLine.strip
    t = stdin.readLine.strip
  var cnt = 0
  for i in 0..<s.len:
    if s[i] == t[i]:
      cnt += 1
  echo cnt
main()
