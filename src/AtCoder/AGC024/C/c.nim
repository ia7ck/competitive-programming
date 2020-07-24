import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  if a[0] != 0:
    echo -1
    return
  var
    r = n - 1
    ans:int64 = 0
  for i in countdown(n - 1, 1):
    if a[i] > i + 1:
      echo -1
      return
    if a[i - 1] + 1 < a[i]:
      echo -1
      return
    if a[i - 1] >= a[i]:
      ans += a[r]
      r = i - 1
  ans += a[r]
  echo ans
main()
