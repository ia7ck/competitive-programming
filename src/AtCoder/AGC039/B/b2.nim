import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = newSeqWith(n, stdin.readLine.strip)
  var ans = -1
  for i in 0..<n:
    var
      seen = newSeq[bool](n)
      vs = @[i]
      ok = true
      c = 0
    seen[i] = true
    while vs.len > 0:
      c += 1
      var nxt = newSeq[int]()
      for v in vs:
        for w in 0..<n:
          if not seen[w] and a[v][w] == '1':
            seen[w] = true
            nxt.add(w)
      for v in nxt:
        for w in nxt:
          if a[v][w] == '1':
            ok = false
      if not ok:
        break
      swap(vs, nxt)
    if ok:
      ans = max(ans, c)
  echo ans
main()
