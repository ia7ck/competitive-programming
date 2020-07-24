import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read()

  if n.count('7') >= 1:
    echo "Yes"
  else:
    echo "No"

main()
