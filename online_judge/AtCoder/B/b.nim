import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc isLowerAscii(c: char): bool =
  return 'a' <= c and c <= 'z'

proc isUpperAscii(c: char): bool =
  return 'A' <= c and c <= 'Z'

proc toLowerAscii(c: char): char =
  doAssert(c.isUpperAscii)
  return (char)((c.ord - 'A'.ord) + 'a'.ord)

proc toUpperAscii(c: char): char =
  doAssert(c.isLowerAscii)
  return (char)((c.ord - 'a'.ord) + 'A'.ord)

proc main() =
  let s = read()

  if s.count('_') == s.len:
    echo s
    return
  var
    ul = 0
    ur = 0
  for i in 0..<s.len:
    if s[i] != '_':
      break
    ul += 1
  for i in countdown(s.len - 1, 0):
    if s[i] != '_':
      break
    ur += 1
  var t = s
  if ul >= 1:
    t = s[ul..^1]
  # echo t
  if ur >= 1:
    t = t[0..<(t.len - ur)]
  # echo t
  # t.removePrefix({'_'})
  # t.removeSuffix({'_'})
  if t.count('_') == 0:
    if not t[0].isLowerAscii:
      echo s
      return
    else:
      var u = ""
      for i in 0..<t.len:
        if t[i].isUpperAscii:
          u &= $('_' & t[i].toLowerAscii)
        else:
          u &= $t[i]
      echo '_'.repeat(ul) & u & '_'.repeat(ur)
  else:
    for c in t:
      if c.isUpperAscii:
        echo s
        return
    let words = t.split('_')
    for w in words:
      if w.len == 0:
        echo s
        return
      if not w[0].isLowerAscii:
        echo s
        return
    var camels = newSeq[string]()
    for i, w in words:
      var v = w
      if i >= 1:
        v[0] = v[0].toUpperAscii
      camels.add(v)
    # echo camels
    echo '_'.repeat(ul) & camels.join("") & '_'.repeat(ur)
main()
# a_bc_de
# _a1__b2_c__
