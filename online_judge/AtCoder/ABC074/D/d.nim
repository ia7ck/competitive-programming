import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = newSeqWith(n, stdin.readLine.strip.split.map(parseBiggestInt))
  var s: int64 = 0
  for i in 0..<n:
    for j in 0..<i:
      var need = true
      for k in 0..<n:
        if i == k or j == k:
          continue
        if a[i][k] + a[k][j] < a[i][j]:
          echo -1
          return
        if a[i][k] + a[k][j] == a[i][j]:
          need = false
      if need:
        s += a[i][j]
  echo s
main()
