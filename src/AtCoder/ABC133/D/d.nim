import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseBiggestInt)

  var b = newSeq[int64](n)
  b[^1] = a[^1]
  for i in 0..<(n - 1):
    if i mod 2 == 0:
      b[^1] -= a[i]
    else:
      b[^1] += a[i]
  for i in countdown(n - 2, 0):
    b[i] = a[i] * 2 - b[i + 1]
  echo b.mapIt($it).join(" ")
main()
