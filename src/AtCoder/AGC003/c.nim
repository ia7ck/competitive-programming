import strutils, sequtils, algorithm
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = newSeqWith(n, stdin.readLine.strip.parseInt)
  type P = tuple[e, i: int]
  var p = newSeq[P]()
  for i in 0..<n:
    p.add((a[i], i))
  p.sort(cmp)
  var c = 0
  for j, ei in p:
    if j mod 2 != ei.i mod 2:
      c += 1
  echo c div 2
main()
