import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    s = read()

  let
    rr = s.count('R')
    gg = s.count('G')
    bb = s.count('B')

  echo (rr mod 2 + gg mod 2 + bb mod 2)
main()
