import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt

  let r = n mod 10
  if r == 2 or r == 4 or r == 5 or r == 7 or r == 9:
    echo "hon"
  elif r == 0 or r == 1 or r == 6 or r == 8:
    echo "pon"
  else:
    echo "bon"

main()
