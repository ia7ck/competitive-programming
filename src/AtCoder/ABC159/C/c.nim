import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let x = read().parseFloat

  let v = x * x * x / 27
  echo v.formatFloat
main()
