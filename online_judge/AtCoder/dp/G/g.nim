import strutils, sequtils

var memo: seq[int]
proc f(g: seq[seq[int]], i: int): int =
  if memo[i]>=0:
    result = memo[i]
  else:
    if g[i].len == 0:
      result = 0
    else:
      for j in g[i]:
        result = max(result, f(g, j))
      result = result+1
    memo[i] = result

proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    xys = (0..<m).mapIt(stdin.readLine.split.map(parseInt))
  var g = newSeqWith(n, newSeq[int]())
  for xy in xys:
    g[xy[0]-1].add(xy[1]-1)
  memo = newSeqWith(n, -1)
  var ans = 0
  for i in 0..<n:
    ans = max(ans, f(g, i))
  echo ans
main()
