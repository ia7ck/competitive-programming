import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = (0..<n).mapIt(stdin.readLine.strip)

  var ans = -1
  for i in 0..<n:
    var
      v = @[i]
      cnt = 1
      k = 1
      ok = true
      seen = newSeq[bool](n)
    seen[i] = true
    while cnt < n:
      var u = newSeq[int]()
      for it in v:
        for nxt in 0..<n:
          if seen[nxt]: continue
          if a[it][nxt] == '1':
            u.add(nxt)
            seen[nxt] = true
            cnt += 1
      for i in u:
        for j in u:
          if a[i][j] == '1':
            ok = false
            break
        if not ok: break
      if not ok:
        break
      k += 1
      swap(u, v)
    if ok:
      ans = max(ans, k)
  echo ans
main()
