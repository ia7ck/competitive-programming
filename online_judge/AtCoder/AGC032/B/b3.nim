import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt

  var edges = newSeq[(int, int)]()
  for i in 1..n:
    for j in (i + 1)..n:
      if n mod 2 == 1:
        if j != n - i:
          edges.add((i, j))
      else:
        if j != n - i + 1:
          edges.add((i, j))
  proc judge(): bool =
    var s = newSeq[int](n)
    for e in edges:
      let (a, b) = e
      s[a - 1] += b
      s[b - 1] += a
    return s.allIt(it == s[0])

  echo edges.len
  for e in edges:
    let (a, b) = e
    echo a, " ", b
  doAssert(judge())
main()
