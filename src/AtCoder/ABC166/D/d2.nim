import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc pow(a, x: int64): int64 =
  var res: int64 = 1
  for i in 1..x:
    res = res * a
  return res

proc main() =
  let x = read().parseBiggestInt
  var p = newSeq[(int, int64)]()
  for i in 0..4000:
    p.add((i, pow(i, 5)))
    p.add((-1 * i, -1 * pow(i, 5)))
  for (a, a5) in p:
    for (b, b5) in p:
      if a5 - b5 == x:
        echo a, " ", b
        return
  doAssert false
main()
