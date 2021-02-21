import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)
  var acc = newSeq[int](n + 1)
  for i in 0..<n:
    acc[i + 1] = gcd(acc[i], a[i])
  var cca = newSeq[int](n + 1)
  for i in countdown(n - 1, 0):
    cca[i] = gcd(cca[i + 1], a[i])
  var mx = 1
  for i in 0..<n:
    mx = max(mx, gcd(acc[i], cca[i + 1]))
  echo mx

main()
