import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let x = read().parseBiggestInt

  var a: int64 = 100
  for i in 1..4000:
    a = a + a div 100
    if a >= x:
      echo i
      break
main()
