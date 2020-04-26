import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let a, b, c, d = read().parseInt

  let
    x = (c + b - 1) div b
    y = (a + d - 1) div d
  if x <= y:
    echo "Yes"
  else:
    echo "No"

main()
