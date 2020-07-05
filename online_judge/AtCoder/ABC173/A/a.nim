import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt

  if n mod 1000 == 0:
    echo 0
  else:
    echo (1000 - n mod 1000)

main()
