import strutils, sequtils, algorithm
proc main() =
  let n = stdin.readLine.strip.parseInt + 1
  var a = stdin.readLine.strip.split.map(parseInt)
  a.add(0)
  a.sort(cmp[int])
  for i in 0..12:
    if a.filterIt(it == i).len >= 3:
      echo 0
      return
  var
    b = newSeq[int]()
    c = newSeq[int]()
  for i in 0..12:
    if a.filterIt(it == i).len == 1:
      b.add(i)
    if a.filterIt(it == i).len == 2:
      c.add(i)
      c.add(24 - i)
  let m = b.len
  var mx = 0
  for bits in 0..<(1 shl m):
    var d = c
    for i in 0..<m:
      if ((bits shr i) and 1) == 1:
        d.add(b[i])
      else:
        d.add(24 - b[i])
    var s = 24
    for i in 0..<n:
      for j in 0..<i:
        var t = abs(d[i] - d[j])
        s = min(s, min(t, 24 - t))
    mx = max(mx, s)
  echo mx
main()
