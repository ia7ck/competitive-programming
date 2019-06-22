import strutils, sequtils, algorithm, future

proc main() =
  let n = stdin.readLine.strip.parseInt
  type P = tuple[a: int64, b: int64]
  var items = newSeq[P]()
  for i in 0..<n:
    var a, b: int64
    (a, b) = stdin.readLine.strip.split.map(parseBiggestInt)
    items.add((a, b))

  items.sort((x, y) => (cmp(x.b, y.b)))
  var t: int64 = 0
  for it in items:
    if t + it.a <= it.b:
      t += it.a
    else:
      echo "No"
      return
  echo "Yes"
main()
