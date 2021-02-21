import strutils, sequtils

proc main() =
  var h, w: int
  (h, w) = stdin.readLine.strip.split.map(parseInt)
  let a = (0..<h).mapIt(stdin.readLine.strip)

  var
    left = newSeqWith(h, newSeq[int](w))
    right = newSeqWith(h, newSeq[int](w))
    up = newSeqWith(h, newSeq[int](w))
    down = newSeqWith(h, newSeq[int](w))
  for i in 0..<h:
    var c = 0
    for j in 0..<w:
      if a[i][j] == '#':
        c = 0
      else:
        c += 1
      left[i][j] = c
    c = 0
    for j in countdown(w - 1, 0):
      if a[i][j] == '#':
        c = 0
      else:
        c += 1
      right[i][j] = c
  for j in 0..<w:
    var c = 0
    for i in 0..<h:
      if a[i][j] == '#':
        c = 0
      else:
        c += 1
      up[i][j] = c
    c = 0
    for i in countdown(h - 1, 0):
      if a[i][j] == '#':
        c = 0
      else:
        c += 1
      down[i][j] = c
  var mx = 1
  for i in 0..<h:
    for j in 0..<w:
      mx = max(mx, left[i][j] + right[i][j] + up[i][j] + down[i][j] - 3)
  echo mx
main()
