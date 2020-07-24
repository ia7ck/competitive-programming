import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let x, y = read().parseInt

  for a in 0..100:
    for b in 0..100:
      if a + b == x and a * 2 + b * 4 == y:
        echo "Yes"
        return
  echo "No"
main()
