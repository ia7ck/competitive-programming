import strutils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  var n = read().parseInt
  if n == 0:
    echo 0
    return
  var s = ""
  while n != 0:
    if n mod (-2) == 0:
      s.add('0')
    else:
      s.add('1')
      n = n - 1
    n = n div (-2)
  s.reverse
  echo s
main()
