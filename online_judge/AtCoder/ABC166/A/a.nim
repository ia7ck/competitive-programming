import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let s = read()
  if s == "ABC":
    echo "ARC"
  else:
    echo "ABC"

main()
