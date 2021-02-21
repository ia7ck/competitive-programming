import strutils, sequtils, algorithm

proc main() =
  let
    nm = stdin.readLine.strip.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
  var xyz = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))

  var ans: int64 = 0
  for a in @[-1, 1]:
    for b in @[-1, 1]:
      for c in @[-1, 1]:
        xyz.sort(proc(p, q: seq[int]): int = system.cmp(
          a * q[0] + b * q[1] + c * q[2],
          a * p[0] + b * p[1] + c * p[2]
        ))
        var sum: int64 = 0
        for p in xyz[0..<m]:
          sum += a * p[0] + b * p[1] + c * p[2]
        ans = max(ans, sum)
  echo ans

main()
