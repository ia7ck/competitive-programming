import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
  var
    mn = 123456789
    ans = 0
  for it in a:
    if it < mn:
      mn = it
      ans += 1
  echo ans
main()
