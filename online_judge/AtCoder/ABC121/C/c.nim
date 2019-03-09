import strutils, sequtils, algorithm

proc main() =
  let
    nm = stdin.readLine.strip.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
  var ab = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))

  ab.sort(proc(x, y: seq[int]): int = cmp(x[0], y[0]))
  var
    k = 0
    ans: int64 = 0
  for it in ab:
    k += it[1]
    ans += (int64)it[0] * it[1]
    if k >= m:
      while k > m:
        ans -= it[0]
        k -= 1
      break
  echo ans
main()
