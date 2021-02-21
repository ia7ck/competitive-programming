import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    k = read().parseBiggestInt
    a = newSeqWith(n, read().parseInt - 1)

  var
    tm = newSeqWith(n, -1)
    p = 0
    i = 0
  while i < k and tm[p] < 0:
    tm[p] = i
    i += 1
    p = a[p]
  if i == k:
    echo p + 1
    return
  let
    len = i - tm[p]
    r = (k - i) mod len
  for _ in 0..<r:
    p = a[p]
  echo p + 1


main()

# 7 19
# 1 2 3 4 5 2 3
# => 4

# 0 1 2 3 4 5
