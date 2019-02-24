import strutils, sequtils, algorithm, tables
proc main() =
  let
    nmq = stdin.readLine.split.map(parseInt)
    (n, m, q) = (nmq[0], nmq[1], nmq[2])
    a = (0..<n).mapIt(stdin.readLine.parseInt)
    b = (0..<m).mapIt(stdin.readLine.parseInt)
    c = (0..<q).mapIt(stdin.readLine.parseInt)
  type P = object
    x: int64
    t: int
  var pts = newSeq[P](0)
  for it in a:
    pts.add(P(x: it, t: 0))
  for it in b:
    pts.add(P(x: it, t: 1))
  for it in c:
    pts.add(P(x: it, t: 2))
  pts.sort(proc(le, ri: P): int = cmp(le.x, ri.x))
  var
    nex_0 = newSeqWith(pts.len, -1)
    nex_1 = newSeqWith(pts.len, -1)
  for i in countdown(pts.len-1, 0):
    if pts[i].t == 0:
      nex_0[i] = i
    elif pts[i].t == 1:
      nex_1[i] = i
    if i > 0:
      nex_0[i-1] = nex_0[i]
      nex_1[i-1] = nex_1[i]
  var
    prev_0 = newSeqWith(pts.len, pts.len+1)
    prev_1 = newSeqWith(pts.len, pts.len+1)
    tab = newTable[int64, int64]()
  for i in 0..<pts.len:
    if pts[i].t == 0:
      prev_0[i] = i
    elif pts[i].t == 1:
      prev_1[i] = i
    else:
      var
        s = newSeq[int64]()
        t = newSeq[int64]()
        mn = high(int64)
      if prev_0[i] < pts.len:
        s.add(pts[prev_0[i]].x)
      if nex_0[i] >= 0:
        s.add(pts[nex_0[i]].x)
      if prev_1[i] < pts.len:
        t.add(pts[prev_1[i]].x)
      if nex_1[i] >= 0:
        t.add(pts[nex_1[i]].x)
      for x0 in s:
        for x1 in t:
          if x0 < pts[i].x and x1 < pts[i].x:
            mn = min(mn, pts[i].x - min(x0, x1))
          elif pts[i].x < x0 and pts[i].x < x1:
            mn = min(mn, max(x0, x1) - pts[i].x)
          else:
            mn = min(mn,
              min(abs(pts[i].x - x0), abs(pts[i].x - x1)) * 2 +
              max(abs(pts[i].x - x0), abs(pts[i].x - x1)))
      tab[pts[i].x] = mn
    if i+1 < pts.len:
      prev_0[i+1] = prev_0[i]
      prev_1[i+1] = prev_1[i]
  for it in c:
    echo tab[it]
main()
