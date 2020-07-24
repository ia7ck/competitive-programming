import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  var
    last = 0
    prev = 1000000000
    ans = 0
  for i in 0..n:
    if i < n and prev >= a[i]:
      discard
    else:
      ans = max(ans, i - last - 1)
      last = i
    if i < n:
      prev = a[i]
  echo ans
main()
