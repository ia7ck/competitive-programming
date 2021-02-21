import strutils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let s = read()

  const m = 2019
  var
    b = 1
    x = 0
    freq = newSeq[int64](m)
    ans: int64 = 0
  freq[0] = 1
  for c in s.reversed:
    let d = c.ord - '0'.ord
    x = (x + d * b) mod m
    ans += freq[x]
    freq[x] += 1
    b = b * 10 mod m
  echo ans
main()
