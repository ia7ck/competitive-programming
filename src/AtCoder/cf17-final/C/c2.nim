import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var d = stdin.readLine.strip.split.map(parseInt)
  d.add(0)
  var 
    e = newSeq[int]()
    f = newSeq[int]()
  for i in 0..12:
    let freq = d.filterIt(it == i).len
    if freq >= 3:
      echo 0
      return
    elif freq == 2:
      e.add(i)
      e.add(24 - i)
    elif freq == 1:
      f.add(i)
  var ans = 0
  for bits in 0..<(1 shl f.len):
    var g = e
    for i in 0..<f.len:
      if ((bits shr i) and 1) == 1:
        g.add(f[i])
      else:
        g.add(24 - f[i])
    var mn = 24
    for i in 0..<g.len:
      for j in 0..<i:
        let x = max(g[i], g[j]) - min(g[i], g[j])
        mn = min(mn, min(x, 24 - x))
    ans = max(ans, mn)
  echo ans
main()
