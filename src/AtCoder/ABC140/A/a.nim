import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var ans = 1
  for i in 0..<3:
    ans = ans * n
  echo ans
main()
