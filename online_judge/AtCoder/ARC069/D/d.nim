import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    s = read()
  proc solve(a, b: bool) =
    var t = newSeq[bool](n)
    t[0] = a
    t[1] = b
    for i in 1..<n:
      let j = (i + 1) mod n
      if s[i] == 'o':
        if t[i]:
          t[j] = t[i - 1]
        else:
          t[j] = not t[i - 1]
      else:
        if t[i]:
          t[j] = not t[i - 1]
        else:
          t[j] = t[i - 1]
    if t[0] == a:
      var ok = false
      if s[0] == 'o' and t[0] and t[n - 1] == t[1]:
        ok = true
      if s[0] == 'o' and (not t[0]) and t[n - 1] != t[1]:
        ok = true
      if s[0] == 'x' and t[0] and t[n - 1] != t[1]:
        ok = true
      if s[0] == 'x' and (not t[0]) and t[n - 1] == t[1]:
        ok = true
      if ok:
        echo t.mapIt(if it: "S" else: "W").join("")
        quit(0)
  solve(true, true)
  solve(true, false)
  solve(false, true)
  solve(false, false)
  echo -1
main()
