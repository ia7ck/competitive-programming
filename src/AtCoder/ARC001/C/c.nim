import strutils, sequtils

const n = 8
var a: seq[string]

proc ok(y, x: int): bool =
  for dy in (-1)..1:
    for dx in (-1)..1:
      if dy == 0 and dx == 0:
        continue
      var k = 1
      while true:
        let
          to_y = y + dy * k
          to_x = x + dx * k
        if to_y < 0 or to_y >= n or to_x < 0 or to_x >= n:
          break
        if a[to_y][to_x] == 'Q':
          return false
        k += 1
  return true

proc dfs(pos, k: int): bool =
  if k == n:
    return true
  if pos == n * n:
    return false
  for p in pos..<(n * n):
    let
      y = p div n
      x = p mod n
    if a[y][x] == 'Q':
      continue
    if ok(y, x):
      a[y][x] = 'Q'
      if dfs(p + 1, k + 1):
        return true
      a[y][x] = '.'
  return false

proc main() =
  a = (0..<n).mapIt(stdin.readLine.strip)
  for i in 0..<n:
    for j in 0..<n:
      if a[i][j] == 'Q':
        if not ok(i, j):
          echo "No Answer"
          return
  if dfs(0, 3):
    for row in a:
      echo row
  else:
    echo "No Answer"
main()
