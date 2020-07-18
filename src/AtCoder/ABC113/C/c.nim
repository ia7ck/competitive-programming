import strutils, sequtils, algorithm
proc printf(formatstr: cstring) {.importc: "printf", varargs,
  header: "<stdio.h>".}
proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
  var a = (0..<m).mapIt(stdin.readLine.split.map(parseInt))
  for i in 0..<m:
    a[i].add(i)
  a.sort do (b, c: seq[int])->int:
    result = cmp(b[0], c[0])
    if result==0:
      result = cmp(b[1], c[1])
  var
    j = 0
    p = 0
    ans = newSeqWith(m, newSeq[int](0))
  for b in a:
    if b[0]!=p:
      j = 1
    else:
      j+=1
    ans[b[2]] = @[b[0], j]
    p = b[0]

  for b in ans:
    printf("%06d%06d\n", b[0], b[1])
main()
