import strutils, sequtils, math, future

proc main() =
  let n = stdin.readLine.strip.parseInt
  var ans = newSeq[int](n)
  ans[0] = 3
  for i in 1..<n:
    ans[i] = 1
  if ans.sum mod 3 == 0:
    ans[n - 1] += 1
  elif ans.sum mod 3 == 2:
    ans[n - 1] += 2
  echo ans.map(it => $it).join(" ")
main()
