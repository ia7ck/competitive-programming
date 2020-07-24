import strutils, sequtils
proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var
    ok = newSeq[bool](n)
    pena = newSeq[int](n)
  for i in 0..<m:
    let
      ps = stdin.readLine.strip.split
      p = ps[0].parseInt - 1
      s = ps[1][0]
    if ok[p]: continue
    if s == 'W':
      pena[p] += 1
    if s == 'A':
      ok[p] = true
  var sm = 0
  for i in 0..<n:
    if ok[i]:
      sm += pena[i]
  echo ok.filterIt(it).len, " ", sm
main()
