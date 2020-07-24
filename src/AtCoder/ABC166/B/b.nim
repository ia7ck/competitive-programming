import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, k = read().parseInt
  var cnt = newSeq[int](n)
  for i in 0..<k:
    let d = read().parseInt
    let a = newSeqWith(d, read().parseInt)
    for j in a:
      cnt[j - 1] += 1
  echo cnt.count(0)
main()
