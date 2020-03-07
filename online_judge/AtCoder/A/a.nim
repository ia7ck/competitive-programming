import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let s = read()
  if s == "AAA" or s == "BBB":
    echo "No"
  else:
    echo "Yes"

main()
