import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  # PD?D??P
  # PDDDDDP
  # => 1 + 5
  let s = read()
  echo s.replace("?", "D")

main()
