import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, k = read().parseInt

  if (n - 2) * (n - 1) div 2 < k:
    echo -1
    return
  var edges = newSeq[(int, int)]()
  for i in 2..n:
    edges.add((1, i))
  var t = (n - 2) * (n - 1) div 2
  for i in 2..n:
    for j in (i + 1)..n:
      if t > k:
        edges.add((i, j))
        t -= 1
  echo edges.len
  for e in edges:
    let (a, b) = e
    echo a, " ", b

main()
