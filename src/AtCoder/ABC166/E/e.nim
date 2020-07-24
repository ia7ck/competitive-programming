import strutils, tables, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
  var
    t = initTable[int64, int64]()
    ans: int64 = 0
  for i in 0..<n:
    let x = i + 1 - a[i]
    if t.hasKey(x):
      ans += t[x]
    let y = i + 1 + a[i]
    if t.hasKey(y):
      t[y] += 1
    else:
      t[y] = 1
  echo ans
main()
