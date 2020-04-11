import strutils, sequtils, tables

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var
    freq = initTable[int64, int64]()
    s: int64 = 0
    ans: int64 = 0
  for it in a:
    s += it
    if s mod m == 0:
      ans += 1
    let x = (((m - s) mod m) + m) mod m
    if freq.hasKey(x):
      ans += freq[x]
      freq[x] += 1
    else:
      freq[x] = 1
  echo ans
main()
