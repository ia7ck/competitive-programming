import strutils, sequtils, math

proc main() =
  let n = stdin.readLine.strip.parseInt
  var
    a = stdin.readLine.strip.split.map(parseBiggestInt)
    b = stdin.readLine.strip.split.map(parseBiggestInt)

  var ans: int64 = 0
  for i in 0..<n:
    ans += min(a[i], b[i])
    b[i] = max(0, b[i] - a[i])
    ans += min(a[i + 1], b[i])
    a[i + 1] = max(0, a[i + 1] - b[i])
  echo ans
main()
