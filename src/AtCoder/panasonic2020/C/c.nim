import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let a, b, c = read().parseBiggestInt
  # a + sqrt(a) * sqrt(b) * 2 + b < c,
  # sqrt(a) * sqrt(b) * 2 < c - b - a

  if c - b - a < 0:
    echo "No"
  elif a * b * 4 < (c - b - a) * (c - b - a):
    echo "Yes"
  else:
    echo "No"
main()
