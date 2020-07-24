import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var ans = n div 2
  if n mod 2 == 0:
    ans -= 1
  echo ans
main()
