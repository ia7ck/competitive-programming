import strutils, sequtils, algorithm

proc main() =
  let
    s = stdin.readLine
    gxgy = stdin.readLine.split.map(parseInt)
    (gx, gy) = (gxgy[0], gxgy[1])

  var
    ops = newSeq[int](len(s))
    idxs = @[0, 1, 2, 3]
    dirs = @[@[0, 1], @[0, -1], @[1, 0], @[-1, 0]]
  for i in 0..<len(s):
    ops[i] = s[i].ord-'W'.ord
  var can = false
  while true:
    var cur = @[0, 0]
    for op in ops:
      if cur[0]==gx and cur[1]==gy:
        can = true
      cur[0] += dirs[idxs[op]][0]
      cur[1] += dirs[idxs[op]][1]
    if cur[0]==gx and cur[1]==gy:
      can = true
    if not idxs.nextPermutation():
      break
  if can:
    echo "Yes"
  else:
    echo "No"
main()
