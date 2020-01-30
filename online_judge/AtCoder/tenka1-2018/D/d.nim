import strutils, sequtils, tables
proc main() =
  let n = stdin.readLine.strip.parseInt
  var
    k = 2
    ok = false
  while k * (k - 1) div 2 <= n:
    if k * (k - 1) div 2 == n:
      ok = true
      break
    k += 1
  if not ok:
    echo "No"
    return
  echo "Yes"

  var sets = newSeq[seq[int]]()
  sets.add(@[1])
  sets.add(@[1])
  for i in 3..k:
    let
      a = (i - 1) * (i - 2) div 2 + 1
      b = i * (i - 1) div 2
    doAssert b - a + 1 == sets.len
    for j in 0..<sets.len:
      sets[j].add(a + j)
    sets.add((a..b).mapIt(it))
  echo k
  for s in sets:
    stdout.write s.len, " "
    echo s.mapIt($it).join(" ")
main()
