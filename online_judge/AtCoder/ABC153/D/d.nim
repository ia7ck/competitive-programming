import strutils, sequtils
proc main() =
  var n = stdin.readLine.strip.parseBiggestInt
  var
    ans: int64 = 0
    p: int64 = 1
  while n > 0:
    n = n div 2
    ans += p
    p = p * 2
  echo ans
main()
