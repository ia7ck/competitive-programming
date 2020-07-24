import strutils, sequtils

proc main() =
  var h, w, k: int
  (h, w, k) = stdin.readLine.strip.split.map(parseInt)
  var c = newSeq[string](h)
  for i in 0..<h:
    c[i] = stdin.readLine.strip
  var a = newSeqWith(h, newSeq[int](w))
  var o = 1
  for i in 0..<h:
    for j in 0..<w:
      if c[i][j] == '#':
        a[i][j] = o
        var dj = 1
        while j - dj >= 0 and c[i][j - dj] == '.':
          a[i][j - dj] = o
          dj += 1
        dj = 1
        while j + dj < w and c[i][j + dj] == '.':
          a[i][j + dj] = o
          dj += 1
        o += 1
  for i in 0..<h:
    for j in 0..<w:
      if a[i][j] > 0:
        var di = 1
        while i - di >= 0 and a[i - di][j] == 0:
          a[i - di][j] = a[i][j]
          di += 1
        di = 1
        while i + di < h and a[i + di][j] == 0:
          a[i + di][j] = a[i][j]
          di += 1
  echo a.mapIt(it.mapIt($it).join(" ")).join("\n")
main()
