import strutils, sequtils, algorithm

proc printf(formatstr: cstring) {.importc: "printf", varargs,
  header: "<stdio.h>".}

let
  nk = stdin.readLine.split.map(parseInt)
  (n, k) = (nk[0], nk[1])
var r = stdin.readLine.split.map(parseInt)

r.sort do (a, b: int)->int:
  result = cmp(a, b)

var cur = 0.0
for i in (n-k)..<n:
  cur = (cur + float(r[i])) / 2.0

printf("%.18f\n", cur)
