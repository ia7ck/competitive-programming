import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt

  if n mod 360 == 90 or n mod 360 == 270:
    echo "Yes"
  else:
    echo "No"
main()
