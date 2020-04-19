import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, k = read().parseBiggestInt
  var ans: int64 = 0
  for i in k..(n + 1):
    let
      a = (n * (n + 1) div 2 - (n - i) * (n - i + 1) div 2)
      b = (i - 1) * i div 2
    ans += a - b + 1
    ans = ans mod 1000000007
  echo ans
main()
