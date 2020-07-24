import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  var a, b = read().parseInt
  # xoxox
  # ooooo
  # xoxox
  # ooooo
  # ooooo
  # xxxxx
  # xxxxx
  # xxxxx
  a -= 1
  b -= 1
  const n = 100
  var c = newSeqWith(n, newSeq[char](n))
  for i in 0..<n:
    for j in 0..<n:
      if i < n div 2:
        c[i][j] = '.'
      else:
        c[i][j] = '#'
  for i in 0..<(n div 2):
    for j in 0..<n:
      if i mod 2 == 0 and (i + j) mod 2 == 0 and b > 0:
        c[i][j] = '#'
        b -= 1
  for i in (n div 2 + 2)..<n:
    for j in 0..<n:
      if i mod 2 == 0 and (i + j) mod 2 == 0 and a > 0:
        c[i][j] = '.'
        a -= 1
  echo n, " ", n
  for r in c:
    echo r.mapIt($it).join("")
main()
