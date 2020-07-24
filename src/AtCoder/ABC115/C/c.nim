import strutils, sequtils, algorithm
proc main() =
  let
    nk = stdin.readLine.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
  var h = (0..<n).mapIt(stdin.readLine.parseInt)
  h.sort(system.cmp)
  var mn = 1000000000
  for i in 0..n-k:
    mn = min(mn, h[i+k-1]-h[i])
  echo mn
main()
