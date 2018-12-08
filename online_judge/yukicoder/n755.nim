import strutils, sequtils

proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
    a = (0..<m).mapIt(stdin.readLine.split.map(parseInt))
  var acc = newSeqWith(m+1, newSeq[int64](m+1))
  for i in 1..m:
    for j in 1..m:
      acc[i][j] = a[i-1][j-1]+acc[i][j-1]+(acc[i-1][j]-acc[i-1][j-1])
  for _ in 0..<n:
    let
      yx = stdin.readLine.split.map(parseInt)
      (y, x) = (yx[0], yx[1])
    var tot = 0
    for sy in 1..m:
      for sx in 1..m:
        for gy in sy..m:
          for gx in sx..m:
            if sy<=y and y<=gy and sx<=x and x<=gx:
              if acc[gy][gx]-acc[gy][sx-1]-acc[sy-1][gx]+acc[sy-1][sx-1]==0:
                tot+=1
    echo tot
main()
