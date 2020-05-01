import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  type P = tuple[x, y: int]
  var pts = newSeq[P]()
  for i in 0..<n:
    let x, y = read().parseInt
    pts.add((x, y))

  var ans = 0
  for p in pts:
    for q in pts:
      var cnt = 2
      if p == q:
        continue
      for r in pts:
        if p == r or q == r:
          continue
        if (r.y - p.y) * (r.x - q.x) == (r.y - q.y) * (r.x - p.x):
          cnt += 1
      ans = max(ans, cnt)
  echo ans
main()
