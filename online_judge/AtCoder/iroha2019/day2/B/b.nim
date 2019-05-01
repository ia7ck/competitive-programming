import strutils, sequtils

proc sign(x: int64): int =
  doAssert(x != 0)
  if x < 0:
    return -1
  else:
    return 1

proc f(x1, y1, x2, y2: int64): int64 =
  return x1 * y2 - y1 * x2

proc main() =
  var x, y, a, b, sx, sy, tx, ty: int64
  (x, y) = stdin.readLine.strip.split.map(parseInt)
  (a, b) = stdin.readLine.strip.split.map(parseInt)
  (sx, sy) = stdin.readLine.strip.split.map(parseInt)
  (tx, ty) = stdin.readLine.strip.split.map(parseInt)

  let
    px = x
    py = b - a
    qx = sx
    qy = sy - a
    rx = tx
    ry = ty - a
  if sign(f(px, py, qx, qy)) * sign(f(px, py, rx, ry)) < 0:
    echo "Yes"
  else:
    echo "No"
main()
