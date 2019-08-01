import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = (0..<n).mapIt(stdin.readLine.strip.split)
  var cand = newSeq[int]()
  for j in 0..<n:
    var nya = 0
    for i in 0..<n:
      if a[i][j] == "nyanpass":
        nya += 1
    if nya + 1 == n:
      cand.add(j + 1)
  if cand.len == 1:
    echo cand[0]
  else:
    echo "-1"

main()
