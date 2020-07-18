import strutils, tables

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var t = initTable[string, bool]()
  for i in 0..<n:
    let s = read()
    t[s] = true
  echo t.len
main()

