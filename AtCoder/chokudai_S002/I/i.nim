import strutils, sequtils, future, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  type P = tuple[a: int, b: int, i: int]
  var items = newSeq[P](n)
  for i in 0..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    items[i] = (a, b, i)
  if n == 1:
    echo 1
    return
  items.sort((x, y) => cmp((y.a + x.b - 1) div x.b, (x.a + y.b - 1) div y.b))
  let x = items[0]
  var ok = true
  for i in 1..<n:
    let y = items[i]
    if ((y.a + x.b - 1) div x.b) >= ((x.a + y.b - 1) div y.b):
      ok = false
      break
  if ok:
    echo x.i + 1
  else:
    echo -1
main()
