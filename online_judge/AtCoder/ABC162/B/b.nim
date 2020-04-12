import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt

  var ans: int64 = 0
  for i in 1..n:
    if i mod 3 != 0 and i mod 5 != 0:
      ans += i
  echo ans
main()
