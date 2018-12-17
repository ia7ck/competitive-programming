import strutils, sequtils
proc printf(formatstr: cstring) {.importc: "printf", varargs,
  header: "<stdio.h>".}

proc main() =
  let
    nd = stdin.readLine.split.map(parseInt)
    (n, d) = (nd[0], nd[1])
    xy = stdin.readLine.split.map(parseInt)
  var
    (x, y) = (xy[0], xy[1])
  if x mod d != 0 or y mod d != 0:
    echo 0
    return
  x = x.abs div d
  y = y.abs div d

  var cmb = newSeqWith(n+1, newSeq[float64](n+1))
  cmb[0][0] = 1
  for j in 1..n:
    cmb[j][0] = cmb[j-1][0]/2.0
  for i in 1..n:
    for j in 1..n:
      cmb[i][j] = (cmb[i-1][j-1]+cmb[i-1][j])/2
  var ans = 0.0
  for i in 0..n:
    let j = n-i
    if i>=x and (i-x) mod 2 == 0 and j>=y and (j-y) mod 2 == 0:
      let
        r = (i-x) div 2
        u = (j-y) div 2
      ans+=cmb[i][r]*cmb[j][u]*cmb[n][i]
  printf("%.18f\n", ans)

main()
