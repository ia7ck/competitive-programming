import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let s = read()

  if s == "{}":
    echo "dict"
    return
  var
    dep = 0
    last = 1
  var a = newSeq[string]()
  for i in 0..<s.len:
    if s[i] == '{':
      dep += 1
    elif s[i] == '}':
      dep -= 1
    elif dep == 1 and s[i] == ',':
      let t = s[last..<i]
      a.add(t)
      last = i + 1
  if last < s.len - 1:
    a.add(s[last..<s.len - 1])
  for t in a:
    var d = 0
    for c in t:
      if c == '{':
        d += 1
      elif c == '}':
        d -= 1
      elif d == 0 and c == ':':
        echo "dict"
        return
  echo "set"
main()
