import strutils, sequtils
proc main() =
  var n, a, b: int64
  (n, a, b) = stdin.readLine.strip.split.map(parseBiggestInt)

  if (b - a) mod 2 == 0:
    echo ((b - a) div 2)
  else:
    var ans = min(a - 1 + (b - a) div 2 + 1, n - b + (b - a) div 2 + 1)
    # 1, b - a + 1
    # a + n - b, n
    echo ans
main()
