import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt).mapIt(it - 1)
    b = stdin.readLine.strip.split.map(parseInt)
    c = stdin.readLine.strip.split.map(parseInt)
  var score = newSeqWith(n, newSeq[int](n))
  for i in 0..<(n - 1):
    score[i][i + 1] = c[i]
  var ans = b.sum
  for i in 1..<n:
    ans += score[a[i - 1]][a[i]]
  echo ans
main()
