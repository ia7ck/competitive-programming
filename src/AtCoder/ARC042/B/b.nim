import strutils, sequtils, future, algorithm, math
proc main() =
  var cx, cy: float64
  (cx, cy) = stdin.readLine.strip.split.map(parseFloat)
  let n = stdin.readLine.strip.parseInt
  type P = tuple[x, y: float64]
  var pts = newSeq[P]()
  for i in 0..<n:
    let xy = stdin.readLine.strip.split.map(parseFloat)
    pts.add((xy[0], xy[1]))
  pts.sort((p, q) => cmp(arctan2(p.y, p.x), arctan2(q.y, q.x)))
  var ans: float64 = 1e18
  for i in 0..<n:
    let
      p = pts[i]
      q = pts[(i + 1) mod n]
    # https://ja.wikipedia.org/wiki/点と直線の距離#直線が通る2点によって定義された直線
    var d = abs((q.y - p.y) * cx - (q.x - p.x) * cy + q.x * p.y - q.y * p.x)
    d = d / hypot(q.y - p.y, q.x - p.x)
    # 辺までの距離
    ans = min(ans, d)
    # 頂点までの距離 (いる?)
    ans = min(ans, hypot(p.y - cy, p.x - cx))
  echo ans.formatFloat
main()
