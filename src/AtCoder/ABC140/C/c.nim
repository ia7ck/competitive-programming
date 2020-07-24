import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    b = stdin.readLine.strip.split.map(parseInt)
  var ans: int64 = b[0]
  for i in 1..<(n - 1):
    ans += min(b[i - 1], b[i])
  ans += b[n - 2]
  echo ans
main()
