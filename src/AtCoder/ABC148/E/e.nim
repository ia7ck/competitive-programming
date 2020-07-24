import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseBiggestInt
  if n < 2 or n mod 2 == 1:
    echo 0
    return
  var
    s: int64 = 0
    f: int64 = 5
  while f <= n:
    s += n div f div 2
    f = f * 5
  echo s
main()
