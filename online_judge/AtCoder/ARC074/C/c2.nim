import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc solve(h, w: int64): int64 =
  var res = int64.high
  for i in (1.int64)..<h:
    let
      s1 = i * w
      s2 = (h - i) * (w div 2)
      s3 = (h - i) * (w - (w div 2))
      t2 = ((h - i) div 2) * w
      t3 = ((h - i) - (h - i) div 2) * w
    res = min(res, max(s1, s2, s3) - min(s1, s2, s3))
    res = min(res, max(s1, t2, t3) - min(s1, t2, t3))
  return res

proc main() =
  let h, w = read().parseBiggestInt

  echo min(solve(h, w), solve(w, h))
main()
