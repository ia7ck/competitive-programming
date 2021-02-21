import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  var x, y, z = read().parseInt
  swap(x, y)
  swap(x, z)
  echo x, " ", y, " ", z

main()
