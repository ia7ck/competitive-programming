import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let (a, b) = (read().parseInt, read().parseInt)
  for x in 0..10000:
    if x * 8 div 100 == a and x * 10 div 100 == b:
      echo x
      return
  echo -1
main()
