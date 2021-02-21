import strutils, sequtils

proc main() =
  var k, x: int
  (k, x) = stdin.readLine.strip.split.map(parseInt)
  var ans = newSeq[int]()
  for i in (x - k + 1)..(x + k - 1):
    ans.add(i)
  echo ans.mapIt($it).join(" ")
main()
