import strutils, algorithm, math, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m = read().parseInt
  type P = tuple[x, y, z: int64]
  var cakes = newSeq[P]()
  for i in 0..<n:
    let x, y, z = read().parseBiggestInt
    cakes.add((x, y, z))
  var ans = int64.low
  for p in @[-1, 1]:
    for q in @[-1, 1]:
      for r in @[-1, 1]:
        cakes.sort(proc(a, b: P): int =
          cmp(b.x * p + b.y * q + b.z * r, a.x * p + a.y * q + a.z * r))
        let
          s = cakes[0..<m].mapIt(it.x).sum
          t = cakes[0..<m].mapIt(it.y).sum
          u = cakes[0..<m].mapIt(it.z).sum
        if s * p >= 0 and t * q >= 0 and u * r >= 0:
          ans = max(ans, cakes[0..<m].mapIt(it.x * p + it.y * q + it.z * r).sum)
  echo ans
main()
