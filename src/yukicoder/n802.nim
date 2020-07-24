import strutils, sequtils

proc gauss(a: var seq[seq[int]]): int =
  let (n, m) = (a.len, a[0].len)
  var rk = 0
  for j in 0..<(m - 1):
    var pv = -1
    for i in rk..<n:
      if a[i][j] == 1:
        pv = i
        break
    if pv >= 0:
      swap(a[rk], a[pv])
      for i in (rk + 1)..<n:
        if a[i][j] == 1:
          for j2 in j..<m:
            a[i][j2] = a[i][j2] xor a[rk][j2]
      rk += 1
  for i in rk..<n:
    if a[i][m - 1] > 0:
      rk = -1
      break
  return rk

proc main() =
  let
    nmx = stdin.readLine.strip.split.map(parseInt)
    (n, m, x) = (nmx[0], nmx[1], nmx[2])
    a = stdin.readLine.strip.split.map(parseInt)
    lrs = (0..<m).mapIt(stdin.readLine.strip.split.map(parseInt))
    b = 33

  var mat = newSeqWith(b + m, newSeq[int](n + 1))
  for i in 0..<b:
    for j in 0..<n:
      if ((a[j] shr i) and 1) == 1:
        mat[i][j] = 1
  for i in 0..<b:
    if ((x shr i) and 1) == 1:
      mat[i][n] = 1
  for i in 0..<m:
    for j in (lrs[i][1] - 1)..(lrs[i][2] - 1):
      mat[b + i][j] = 1
    mat[b + i][n] = lrs[i][0]
  let rk = gauss(mat)
  if rk < 0:
    echo 0
  else:
    var
      ans: int64 = 1
      mo: int64 = 1000000000 + 7
    for _ in 0..<(n - rk):
      ans = ans * 2 mod mo
    echo ans
main()
