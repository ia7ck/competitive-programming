import strutils, sequtils, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  # n = 8, k = 3
  # 8 -> 5 -> 2 -> 1
  # n = 4, k = 10
  # 4 -> 6 -> 4 -> 6 -> ...
  # n = 8, k = 6
  # 8 -> 2 -> 4 -> 2 -> ...

  let n, k = read().parseBiggestInt
  if n mod k == 0:
    echo 0
    return
  if n < k:
    echo min(n, k - n)
    return
  let a = n div k
  echo min(n - k * a, k - (n - k * a))
main()
