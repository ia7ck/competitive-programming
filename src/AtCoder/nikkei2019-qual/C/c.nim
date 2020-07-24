import strutils, sequtils, algorithm

proc main() =
  let n = stdin.readLine.parseInt
  var ab = (0..<n).mapIt(stdin.readLine.split.map(parseBiggestInt))
  ab.sort(proc (x, y: seq[int64]): int = cmp(y[0]+y[1], x[0]+x[1]))
  var ans: int64 = 0
  for i in 0..<n:
    if i mod 2 == 0:
      ans+=ab[i][0]
    else:
      ans-=ab[i][1]
  echo ans
main()
