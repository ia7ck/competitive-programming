import strutils, sequtils, future, algorithm

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[x, y: int64]
  var pts = newSeq[P]()
  for i in 0..<n:
    var x, y: int64
    (x, y) = stdin.readLine.strip.split.map(parseBiggestInt)
    pts.add((x, y))
  pts.sort((p, q) => cmp(p.y, q.y))
  var ans = int64.high
  for i0 in 0..<n:
    for j0 in (i0 + 1)..<n:
      let a = pts[i0..j0].sortedByIt(it.x)
      if a.len < k:
        continue
      let h = pts[j0].y - pts[i0].y
      for i in 0..<a.len:
        if i >= k - 1:
          ans = min(ans, h * (a[i].x - a[i - (k - 1)].x))
  echo ans
main()
