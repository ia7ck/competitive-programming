import strutils, sequtils

proc f(i: int, color: int, c: var seq[int], g: seq[seq[int]]) =
  c[i] = color
  for j in g[i]:
    if c[j]<0:
      f(j, color, c, g)

proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
  var g = newSeqWith(n, newSeqWith(n, 0))
  for _ in 0..<m:
    let ab = stdin.readLine.split.map(parseInt)
    g[ab[0]-1][ab[1]-1] = 1
    g[ab[1]-1][ab[0]-1] = 1
  var mx = 0
  for bit in 0..<(1 shl n):
    var ok = true
    for i in 0..<n:
      if (bit and (1 shl i))>0:
        for j in 0..<n:
          if i!=j and (bit and (1 shl j))>0:
            if g[i][j]==0:
              ok = false
    if ok:
      var cnt = 0
      for i in 0..<n:
        if (bit and (1 shl i))>0: cnt+=1
      mx = max(mx, cnt)
  echo mx

main()
