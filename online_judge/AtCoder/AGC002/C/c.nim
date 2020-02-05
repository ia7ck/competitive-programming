import strutils, sequtils, algorithm
proc main() =
  var n, s: int
  (n, s) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  var j = -1
  for i in 0..<(n - 1):
    if a[i] + a[i + 1] >= s:
      j = i
      break
  if j < 0:
    echo "Impossible"
    return
  echo "Possible"
  var b = newSeq[int]()
  for i in 1..j:
    b.add(i)
  for i in countdown(n - 1, j + 2):
    b.add(i)
  b.add(j + 1)
  echo b.mapIt($it).join("\n")
main()
