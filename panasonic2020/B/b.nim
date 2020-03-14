import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let h, w = read().parseBiggestInt

  if h == 1 or w == 1:
    echo 1
    return
  echo ((h * w + 1) div 2)

main()
