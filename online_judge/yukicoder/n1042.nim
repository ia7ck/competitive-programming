import strutils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let p, q = read().parseFloat
  proc f(x: float): float =
    return p + q * x * log2(x) - x * x
  var
    i = 1.float
    j = int64.high.float / 2
  for tt in 0..100:
    let m = (i + j) / 2
    if f(m) >= 0:
      i = m
    else:
      j = m
  echo i
main()
