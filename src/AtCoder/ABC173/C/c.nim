import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    h, w, k = read().parseInt
    a = newSeqWith(h, read())

  var ans = 0
  for b in 0..<(1 shl h):
    for c in 0..<(1 shl w):
      var s = 0
      for i in 0..<h:
        for j in 0..<w:
          if ((b shr i) and 1) == 1:
            continue
          if ((c shr j) and 1) == 1:
            continue
          if a[i][j] == '#':
            s += 1
      if s == k:
        ans += 1
  echo ans

main()
