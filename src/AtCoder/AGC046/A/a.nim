import strutils, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let x = read().parseInt

  echo (360 div gcd(360, x))

main()
