import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, s = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var k = -1
  for i in 0..<(n - 1):
    if a[i] + a[i + 1] >= s:
      k = i
      break
  if k == -1:
    echo "Impossible"
    return
  echo "Possible"
  var ans = @[k]
  for i in countdown(k - 1, 0):
    ans.add(i)
  for i in (k + 1)..<(n - 1):
    ans.add(i)
  ans.reverse
  echo ans.mapIt($(it + 1)).join("\n")
main()
