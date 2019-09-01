import strutils, sequtils, math, algorithm, future
proc printf(formatstr: cstring) {.importc: "printf", varargs,
  header: "<stdio.h>".}

proc main() =
  let n = stdin.readLine.strip.parseInt
  type V = tuple[x: int64, y: int64, theta: float64]
  var vectors = newSeq[V]()
  for i in 0..<n:
    var x, y: int64
    (x, y) = stdin.readLine.strip.split.map(parseBiggestInt)
    vectors.add((x, y, arctan2(y.float64, x.float64)))

  vectors.sort((u, v) => cmp(u.theta, v.theta))
  for i in 0..<n:
    vectors.add(vectors[i])
  var ans: int64 = 0
  for i in 0..<n:
    let v = vectors[i]
    var
      j = i
      xx: int64 = 0
      yy: int64 = 0
    while j < i + n:
      xx += vectors[j].x
      yy += vectors[j].y
      j += 1
      ans = max(ans, xx * xx + yy * yy)
  printf("%.18f\n", sqrt(ans.float64))
main()
