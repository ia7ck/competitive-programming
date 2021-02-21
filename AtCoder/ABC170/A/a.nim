import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let x = newSeqWith(5, read().parseInt)

  echo x.find(0) + 1

main()
