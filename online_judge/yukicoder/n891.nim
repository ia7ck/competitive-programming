import strutils, sequtils

const mo: int64 = 1000000000 + 7
proc add(a: var int64, b: int64) =
  a = (a + b) mod mo

proc main() =
  var a, b, n: int
  (a, b, n) = stdin.readLine.strip.split.map(parseInt)

  if n <= 1:
    echo n
    return
  var mat = newSeqWith(32, newSeqwith(2, newSeq[int64](2)))
  mat[0][0][0] = a
  mat[0][0][1] = b
  mat[0][1][0] = 1
  mat[0][1][1] = 0
  for i in 1..<32:
    for y in 0..<2:
      for x in 0..<2:
        for t in 0..<2:
          add(mat[i][y][x], mat[i - 1][y][t] * mat[i - 1][t][x] mod mo)
  var v = newSeq[int64](2)
  v[0] = 1
  v[1] = 0
  for i in 0..<32:
    if ((n - 1) and (1 shl i)) > 0:
      var u = newSeq[int64](2)
      for j in 0..<2:
        for t in 0..<2:
          add(u[j], mat[i][j][t] * v[t] mod mo)
      swap(u, v)
  echo v[0]
main()
