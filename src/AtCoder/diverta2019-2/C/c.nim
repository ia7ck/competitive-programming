import strutils, sequtils, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseInt)

  a.sort(system.cmp)
  var
    p = newSeq[int]()
    m = newSeq[int]()
  for it in a:
    if it >= 0:
      p.add(it)
    else:
      m.add(it)
  type P = tuple[x: int, y: int]
  var ans = newSeq[P]()
  if p.len == n:
    var ans = a[1] - a[0]
    for i in 2..<n:
      ans += a[i]
    echo ans
    var s = a[0]
    for i in countdown(n - 1, 1):
      if i > 1:
        echo s, " ", a[i]
      else:
        echo a[i], " ", s
      s -= a[i]
    return
  elif m.len == n:
    var ans = a[n - 1] - a[n - 2]
    for i in 0..<(n - 2):
      ans -= a[i]
    echo ans
    var s = a[n - 1]
    for i in 0..<(n - 1):
      echo s, " ", a[i]
      s -= a[i]
    return
  if m.len > 0:
    m.reverse
  while p.len > 0 and m.len > 0:
    let
      pb = p.pop
      mb = m.pop
    if p.len == 0 and m.len == 0:
      ans.add((pb, mb))
      break
    elif p.len < m.len:
      ans.add((pb, mb))
      p.add(pb - mb)
    else:
      ans.add((mb, pb))
      m.add(mb - pb)
  doAssert(p.len == 0 and m.len == 0 and ans.len == n - 1)
  echo ans[^1].x - ans[^1].y
  echo ans.mapIt($it.x & " " & $it.y).join("\n")
main()
