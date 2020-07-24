import strutils, sequtils

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var s = stdin.readLine.strip
  k -= 1
  if s[k] == 'A':
    s[k] = 'a'
  elif s[k] == 'B':
    s[k] = 'b'
  else:
    s[k] = 'c'
  echo s

main()
