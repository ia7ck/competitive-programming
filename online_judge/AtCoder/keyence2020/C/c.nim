import strutils, sequtils
proc main() =
  var n, k, s: int
  (n, k, s) = stdin.readLine.strip.split.map(parseInt)
  var a = newSeq[int](n)
  const inf = 1000000000
  for i in 0..<n:
    if i < k:
      a[i] = s
    else:
      a[i] = if s == inf: s - 1 else: inf
  echo a.mapIt($it).join(" ")
  discard
main()
