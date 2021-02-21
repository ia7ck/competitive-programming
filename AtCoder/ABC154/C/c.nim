import strutils, sequtils, algorithm
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt).sorted(cmp)
  for i in 1..<n:
    if a[i - 1] == a[i]:
      echo "NO"
      return
  echo "YES"
main()
