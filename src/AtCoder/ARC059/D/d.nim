import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    s = read()
    n = s.len

  # 長さ 2 or 3 のみ考えれば十分
  for i in 0..<(n - 1):
    if s[i] == s[i + 1]:
      echo i + 1, " ", i + 2
      return
  for i in 0..<(n - 2):
    if s[i] == s[i + 1] or s[i] == s[i + 2] or s[i + 1] == s[i + 2]:
      echo i + 1, " ", i + 3
      return
  echo "-1 -1"

main()
