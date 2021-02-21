import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let s = read()

  let ok = s[2] == s[3] and s[4] == s[5]
  if ok:
    echo "Yes"
  else:
    echo "No"

main()
