import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let k, a, b = read().parseInt

  var ok = false
  for i in a..b:
    if i mod k == 0:
      ok = true
  if ok:
    echo "OK"
  else:
    echo "NG"
main()
