import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let k = read().parseBiggestInt

  const n = 50
  var a = newSeq[int64](n)
  for i in 0..<n:
    a[i] = 50 - i
  if k < 50:
    for i in 0..<(n - k.int):
      a[n - i - 1] = 0
  else:
    let
      m = k - 50
      x = (m + n - 1) div n
    for i in 0..<n:
      if i <= (k - 1) mod n:
        a[i] += x
      else:
        a[i] += x - 1
  echo n
  echo a.mapIt($it).join(" ")

main()
