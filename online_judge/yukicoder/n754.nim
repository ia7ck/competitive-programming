import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.parseInt
    a = (0..<n+1).mapIt(stdin.readLine.parseInt)
    b = (0..<n+1).mapIt(stdin.readLine.parseInt)
    mo = 1_000_000_000+7

  var
    s = 0
    t = 0
  for i in 0..n:
    t = (t+b[i]) mod mo
    s = (s+a[n-i]*t) mod mo
  echo s

main()
