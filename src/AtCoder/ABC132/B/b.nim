import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  var ans = 0
  for i in 1..<(n - 1):
    if a[i - 1] < a[i] and a[i] < a[i + 1]:
      ans += 1
    elif a[i - 1] > a[i] and a[i] > a[i + 1]:
      ans += 1
  echo ans

main()
