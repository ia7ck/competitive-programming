import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n - 1, read().parseInt)
  var ans = newSeq[int](n)
  for i in 0..<(n - 1):
    ans[a[i] - 1] += 1
  echo ans.join("\n")
main()
