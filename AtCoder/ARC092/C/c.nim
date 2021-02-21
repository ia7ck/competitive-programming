import strutils, sequtils, algorithm, intsets

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt
  type P = tuple[x, y, t: int]
  var pts = newSeq[P]()
  for i in 0..<(n * 2):
    let x, y = read().parseInt
    pts.add((x, y, i div n))

  pts.sort(cmp)
  var
    s = initIntSet()
    ans = 0
  for p in pts:
    if p.t == 0:
      s.incl(p.y)
    else:
      for y in countdown(p.y - 1, 0):
        if s.contains(y):
          ans += 1
          s.excl(y)
          break
  echo ans
main()
