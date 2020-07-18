import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  var s = read()
  let q = read().parseInt
  var
    rev = false
    a = ""
    b = ""
  for qq in 0..<q:
    let t = read().parseInt
    if t == 1:
      rev = not rev
      swap(a, b)
    else:
      let
        f = read().parseInt
        c = read()
      if f == 1:
        a &= c
      else:
        b &= c
  if rev:
    s.reverse
  a.reverse
  echo a, s, b
main()
