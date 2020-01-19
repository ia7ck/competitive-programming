import strutils, sequtils, algorithm
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    xl = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))
  type P = tuple[left, right: int]
  var a = newSeq[P]()
  for it in xl:
    a.add((it[0] - it[1], it[0] + it[1]))
  let b = a.sortedByIt(it.right)
  var
    cur = -1234567890
    ans = 0
  for it in b:
    if cur <= it.left:
      cur = it.right
      ans += 1
  echo ans
main()
