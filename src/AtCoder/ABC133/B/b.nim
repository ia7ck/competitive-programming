import strutils, sequtils

proc is_square(x: int): bool =
  var i = 1
  while i * i <= x:
    if i * i == x:
      return true
    i += 1
  return false

proc main() =
  var n, d: int
  (n, d) = stdin.readLine.strip.split.map(parseInt)
  let pts = (0..<n).mapIt(stdin.readLine.strip.split.map(parseInt))

  var ans = 0
  for i in 0..<n:
    for j in 0..<i:
      var d2 = 0
      for k in 0..<d:
        d2 += (pts[i][k] - pts[j][k]) * (pts[i][k] - pts[j][k])
      if is_square(d2):
        ans += 1
  echo ans
main()
