import strutils, sequtils, math, future

proc main() =
  let
    nabc = stdin.readLine.split.map(parseInt)
    (n, a, b, c) = (nabc[0], nabc[1], nabc[2], nabc[3])
    y = (0..<n).mapIt(stdin.readLine.parseInt)
  var
    p4 = newSeq[int](n+1)
    ans = high(int)
  p4[0] = 1
  for i in 1..n:
    p4[i] = p4[i-1] * 4
  for i in 0..<p4[n]:
    var abc = newSeqWith(3, newSeq[int]())
    for j in 0..<n:
      let k = (i div p4[j]) mod 4
      if k <= 2:
        abc[k].add(y[j])
    if abc.all(it => it.len > 0):
      var cost = abc.map(it => it.len - 1).sum * 10
      for i, x in [a, b, c]:
        cost += abs(abc[i].sum - x)
      ans = min(ans, cost)
  echo ans
main()
