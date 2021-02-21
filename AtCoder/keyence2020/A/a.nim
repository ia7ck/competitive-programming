import strutils, sequtils
proc main() =
  var
    h = stdin.readLine.strip.parseInt
    w = stdin.readLine.strip.parseInt
    n = stdin.readLine.strip.parseInt
  var
    ans = 0
    s = 0
  if h < w:
    swap(h, w)
  for i in 0..<w:
    s += h
    ans += 1
    if s >= n:
      break
  echo ans

main()
