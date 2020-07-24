import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let s, w = read().parseInt
  if s <= w:
    echo "unsafe"
  else:
    echo "safe"

main()
