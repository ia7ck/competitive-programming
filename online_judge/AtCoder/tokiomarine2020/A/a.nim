import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let s = read()
  echo s[0..<3]

main()
