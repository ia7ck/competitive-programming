import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let c = read()

  if c[0].isUpperAscii:
    echo "A"
  else:
    echo "a"

main()
