import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let a = read().parseInt

  echo a + a * a + a * a * a

main()
