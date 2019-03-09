import strutils, sequtils

proc main() =
  let
    nmc = stdin.readLine.strip.split.map(parseInt)
    (n, m, c) = (nmc[0], nmc[1], nmc[2])
    b = stdin.readLine.strip.split.map(parseInt)
    aa = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))
  var ans = 0
  for a in aa:
    var sum = 0
    for i in 0..<m:
      sum += a[i] * b[i]
    if sum + c > 0:
      ans += 1
  echo ans
main()
