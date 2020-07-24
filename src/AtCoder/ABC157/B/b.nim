import strutils, sequtils

proc main() =
  let n = 3
  var a = newSeqWith(n, newSeq[int](n))
  for i in 0..<n:
    a[i] = stdin.readLine.strip.split.map(parseInt)
  let q = stdin.readLine.strip.parseInt
  let b = newSeqWith(q, stdin.readLine.strip.parseInt)

  var checked = newSeqWith(n, newSeq[bool](n))
  for it in b:
    for i in 0..<n:
      for j in 0..<n:
        if a[i][j] == it:
          checked[i][j] = true
  var bingo = false
  for i in 0..<n:
    var all = true
    # 横
    for j in 0..<n:
      if not checked[i][j]:
        all = false
    if all:
      bingo = true
    all = true
    # 縦
    for j in 0..<n:
      if not checked[j][i]:
        all = false
    if all:
      bingo = true
  # 斜め
  if checked[0][0] and checked[1][1] and checked[2][2]:
    bingo = true
  if checked[0][2] and checked[1][1] and checked[2][0]:
    bingo = true
  if bingo:
    echo "Yes"
  else:
    echo "No"

main()
