import strutils, deques, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(n, read().parseInt)
    s = read()

  var q = initDeque[int]()
  for it in a:
    q.addLast(it)
  for c in s:
    if c == 'L':
      let
        x = q.popFirst
        y = q.popFirst
      q.addFirst(x + y)
      q.addLast(0)
    else:
      let
        x = q.popLast
        y = q.popLast
      q.addLast(x + y)
      q.addFirst(0)
  echo q.mapIt($it).join(" ")
main()
