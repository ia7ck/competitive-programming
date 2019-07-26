import strutils, sequtils, tables, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseBiggestInt)

  a.sort(system.cmp)
  var seen = initTable[int64, bool]()
  var freq = initTable[int64, int]()
  for it in a:
    seen[it] = true
    if freq.hasKey(it):
      freq[it] += 1
    else:
      freq[it] = 1
  if seen.len > 3:
    echo "No"
    return
  if seen.len == 1:
    if a.allIt(it == 0):
      echo "Yes"
    else:
      echo "No"
    return
  if seen.len == 2:
    if a.allIt(it > 0):
      echo "No"
    elif n mod 3 != 0:
      echo "No"
    elif freq[0] != (n div 3):
      echo "No"
    else:
      echo "Yes"
    return
  if seen.len == 3:
    var x: int64 = 0
    var fs = newSeq[int]()
    for k in seen.keys:
      x = x xor k
      fs.add(freq[k])
    if x != 0:
      echo "No"
    elif n mod 3 != 0:
      echo "No"
    elif fs.anyIt(it != (n div 3)):
      echo "No"
    else:
      echo "Yes"

main()
