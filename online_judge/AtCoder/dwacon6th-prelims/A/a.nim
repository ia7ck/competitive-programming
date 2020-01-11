import strutils, sequtils
proc main() =
  let n = stdin.readLine.strip.parseInt
  var
    ss = newSeq[string](n)
    tt = newSeq[int](n)
  for i in 0..<n:
    let st = stdin.readLine.strip.split
    ss[i] = st[0]
    tt[i] = st[1].parseInt
  let x = stdin.readLine.strip

  var
    tot = 0
    seen = false
  for i in 0..<n:
    if seen:
      tot += tt[i]
    if ss[i] == x:
      seen = true
  echo tot
main()
