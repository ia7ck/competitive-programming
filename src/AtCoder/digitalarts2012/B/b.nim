import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let s = read()

  if s == "zzzzzzzzzzzzzzzzzzzz":
    echo "NO"
    return
  if s.count('z') == s.len:
    echo "ay" & s[1..^1]
    return
  if s.len == 1:
    if s == "a":
      echo "NO"
    else:
      echo 'a' & (char)(s[0].ord - 1)
    return
  var h = 0
  for c in s:
    h += c.ord - 'a'.ord + 1
  var t = ""
  while h > 0:
    if h >= 26:
      t &= "z"
      h -= 26
    else:
      t &= $(char)('a'.ord + h - 1)
      h = 0
  if s.len != t.len:
    echo t
    return
  for i in 0..<t.len:
    let u = t[(i + 1)..^1] & t[0..i]
    if u != s:
      echo u
      return
  echo -1
main()

# zze
