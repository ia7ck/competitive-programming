import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip
  var
    last = '?'
    ans = 0
  for i in 0..<n:
    if s[i] != last:
      ans += 1
      last = s[i]
  echo ans
main()
