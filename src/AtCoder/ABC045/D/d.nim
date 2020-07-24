import strutils, tables, math, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    h, w = read().parseBiggestInt
    n = read().parseInt
  var c = initTable[(int, int), int]()
  for i in 0..<n:
    let a, b = read().parseInt
    for i in -1..1:
      for j in -1..1:
        let
          y = a + i
          x = b + j
        if y >= 2 and y <= h - 1 and x >= 2 and x <= w - 1:
          if c.hasKey((y, x)):
            c[(y, x)] += 1
          else:
            c[(y, x)] = 1
  var ans = newSeq[int64](10)
  for v in c.values:
    ans[v] += 1
  ans[0] = (h - 2) * (w - 2) - ans[1..^1].sum
  echo ans.mapIt($it).join("\n")
main()
