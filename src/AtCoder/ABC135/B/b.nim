import strutils, sequtils, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseInt)

  var b = a
  b.sort(system.cmp)
  var ok = a == b
  for i in 0..<(n - 1):
    for j in (i + 1)..<n:
      swap(a[i], a[j])
      if a == b:
        ok = true
      swap(a[i], a[j])
  if ok:
    echo "YES"
  else:
    echo "NO"
main()
