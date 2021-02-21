import strutils, sequtils

proc solve(h, w: int64): int64 =
  if h mod 3 == 0:
    return 0
  var res = int64.high
  for i in 1..(h - 1):
    let
      s = i * (w div 2)
      t = i * (w - w div 2)
      u = (i div 2) * w
      v = (i - i div 2) * w
      z = (h - i) * w
    res = min(res, max(s, t, z) - min(s, t, z))
    res = min(res, max(u, v, z) - min(u, v, z))
  return res

proc main() =
  var h, w: int64
  (h, w) = stdin.readLine.strip.split.map(parseInt)

  echo min(solve(h, w), solve(w, h))
main()
