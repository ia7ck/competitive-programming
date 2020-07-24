import strutils, sequtils, tables, algorithm

let
  dx = @[0, 0, 1, -1]
  dy = @[1, -1, 0, 0]
proc dfs(a: var seq[seq[int]], x, y: int) =
  a[x][y] = -1
  for k in 0..<4:
    let
      nx = x + dx[k]
      ny = y + dy[k]
    if 0 <= nx and nx < a.len and 0 <= ny and ny < a[0].len:
      if a[nx][ny] == 0:
        dfs(a, nx, ny)

proc dedup(a: seq[int]): seq[int] =
  result = newSeq[int]()
  for it in a:
    if result.len == 0:
      result.add(it)
    elif result[^1] != it:
      result.add(it)

proc index(a: seq[int], v: int): int =
  return a.lowerBound(v)

proc main() =
  var w, h: int
  (w, h) = stdin.readLine.strip.split.map(parseInt)
  let n = stdin.readLine.strip.parseInt
  type P = tuple[x1, y1, x2, y2: int]
  var pts = newSeq[P]()
  var
    xs = newSeq[int]()
    ys = newSeq[int]()
  for i in 0..<n:
    var x1, y1, x2, y2: int
    (x1, y1, x2, y2) = stdin.readLine.strip.split.map(parseInt)
    pts.add((x1, y1, x2, y2))
    xs.add(x1)
    xs.add(x2)
    ys.add(y1)
    ys.add(y2)
  xs.add(0)
  xs.add(w)
  ys.add(0)
  ys.add(h)

  xs.sort(system.cmp)
  ys.sort(system.cmp)
  xs = xs.dedup
  ys = ys.dedup
  let
    W = xs.len - 1
    H = ys.len - 1
  var a = newSeqWith(W, newSeq[int](H))
  for p in pts:
    let (x1, y1, x2, y2) = p
    let
      mx1 = xs.index(x1)
      my1 = ys.index(y1)
      mx2 = xs.index(x2)
      my2 = ys.index(y2)
    a[mx1][my1] += 1
    if mx2 < W:
      a[mx2][my1] -= 1
    if my2 < H:
      a[mx1][my2] -= 1
    if mx2 < W and my2 < H:
      a[mx2][my2] += 1
  for i in 0..<W:
    for j in 1..<H:
      a[i][j] += a[i][j - 1]
  for j in 0..<H:
    for i in 1..<W:
      a[i][j] += a[i - 1][j]

  var cnt = 0
  for x in 0..<W:
    for y in 0..<H:
      if a[x][y] == 0:
        dfs(a, x, y)
        cnt += 1
  echo cnt
main()
