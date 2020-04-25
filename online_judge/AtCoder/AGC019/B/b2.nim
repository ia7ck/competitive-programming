import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    A = read()
    n = A.len
  var
    freq = newSeq[int64](26)
    ans: int64 = 1
  for c in A:
    for d in 'a'..'z':
      if c != d:
        ans += freq[d.ord - 'a'.ord]
    freq[c.ord - 'a'.ord] += 1
  echo ans

main()
