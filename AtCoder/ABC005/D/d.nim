import strutils, sequtils

let
  n = stdin.readLine.parseInt
  d = (0..<n).mapIt(stdin.readLine.split.map(parseInt))

var acc = newSeqWith(n+1, newSeqWith(n+1, 0))
for i in 0..<n:
  for j in 0..<n:
    acc[i+1][j+1] = acc[i+1][j] + d[i][j]
for j in 0..<n:
  for i in 0..<n:
    acc[i+1][j+1] += acc[i][j+1]
var b = newSeqWith(n*n+1, 0)
for sy in 0..<n:
  for sx in 0..<n:
    for gy in (sy+1)..n:
      for gx in (sx+1)..n:
        let
          a = (gy-sy)*(gx-sx)
          s = acc[gy][gx] - acc[gy][sx] - acc[sy][gx] + acc[sy][sx]
        b[a] = max(b[a], s)
for i in 1..(n*n):
  b[i] = max(b[i], b[i-1])
let q = stdin.readLine.parseInt
for _ in 0..<q:
  let p = stdin.readLine.parseInt
  echo b[p]
