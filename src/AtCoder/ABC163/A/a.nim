import strutils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let r = read().parseFloat
  echo (r * 2 * PI).formatFloat
main()
