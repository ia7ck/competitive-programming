import strutils, sequtils

const
  N = 1000
  mo = 1_000_000_000+7
var cmb = newSeqWith(N, newSeq[int64](N))
for i in 1..<N:
  cmb[i][0] = 1
  cmb[i][1] = i
  cmb[i][i] = 1
for i in 1..<N:
  for j in 2..<N:
    cmb[i][j] = (cmb[i-1][j]+cmb[i-1][j-1]) mod mo

let
  rc = stdin.readLine.split.map(parseInt)
  (r, c) = (rc[0], rc[1])
  xy = stdin.readLine.split.map(parseInt)
  (x, y) = (xy[0], xy[1])
  dl = stdin.readLine.split.map(parseInt)
  (d, _) = (dl[0], dl[1])

echo cmb[x*y][d]*(r-x+1) mod mo*(c-y+1) mod mo
