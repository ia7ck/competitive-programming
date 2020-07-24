import strutils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc dist(a, b: int): int =
  echo "? ", a + 1, " ", b + 1
  let d = read().parseInt
  return d

proc main() =
  let n = read().parseInt

  var d = newSeq[int](n)
  for i in 1..<n:
    d[i] = dist(0, i)
  let
    mx = d.max
    jj = d.find(mx)
  var ans = 0
  for i in 0..<n:
    if i != jj:
      ans = max(ans, dist(i, jj))
  echo "! ", ans
main()
