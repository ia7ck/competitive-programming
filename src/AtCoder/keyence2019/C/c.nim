import strutils, sequtils, algorithm

proc enough(m: int, diff: seq[int64]): bool =
  var
    need = 0
    neg_sum = 0'i64
  for it in diff:
    if it<0:
      need+=1
      neg_sum+=it*(-1)
  if need>m:
    return false
  var
    rest = m-need
    plus = 0'i64
  for i in 0..<rest:
    plus+=diff[i]
  result = plus>=neg_sum

proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseBiggestInt)
    b = stdin.readLine.split.map(parseBiggestInt)
  if a.foldl(a+b)<b.foldl(a+b):
    echo "-1"
    return
  var all = true
  for i in 0..<n:
    if a[i]<b[i]:
      all = false
  if all:
    echo 0
    return
  var
    (ng, ok) = (0, n)
    diff = newSeq[int64](n)
  for i in 0..<n:
    diff[i] = a[i]-b[i]
  diff.sort(proc(x, y: int64): int = cmp(y, x))
  while ok-ng>1:
    let md = (ok+ng) div 2
    if enough(md, diff):
      ok = md
    else:
      ng = md
  echo ok

main()
