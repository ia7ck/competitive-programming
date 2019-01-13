import strutils, sequtils

let MOD = 1_000_000_000+7
proc madd(a: var int64, b: int64) =
  a = (a+b) mod MOD

proc main() =
  let
    nk = stdin.readLine.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
    a = (0..<n).mapIt(stdin.readLine.split.map(parseBiggestInt))
  var mats = newSeqWith(64, newSeqWith(n, newSeqWith(n, 0'i64)))
  mats[0] = a
  for t in 1..<64:
    for i in 0..<n:
      for j in 0..<n:
        for z in 0..<n:
          madd(mats[t][i][j], mats[t-1][i][z]*mats[t-1][z][j] mod MOD)
  var v = newSeqWith(n, 1'i64)
  for t in 0..<64:
    if (k and (1 shl t))>0:
      var u = newSeqWith(n, 0'i64)
      for i in 0..<n:
        for z in 0..<n:
          madd(u[i], mats[t][i][z]*v[z] mod MOD)
      v = u
  echo v.foldl((a+b) mod MOD)
main()
