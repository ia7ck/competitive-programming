import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var c, d = read().parseBiggestInt
  for i in 1..<n:
    let
      t, a = read().parseBiggestInt
      k = max((t + c - 1) div t, (a + d - 1) div a)
    c = k * t
    d = k * a
  echo c + d
main()
