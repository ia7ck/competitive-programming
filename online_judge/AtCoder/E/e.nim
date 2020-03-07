import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc solve2(s: string): int64 =
  result = 0
  for i in 0..<s.len:
    if (s[i].ord - '0'.ord) mod 2 == 0:
      result += i + 1

proc solve5(s: string): int64 =
  result = 0
  for i in 0..<s.len:
    if s[i] == '5' or s[i] == '0':
      result += i + 1

proc main() =
  let
    n, p = read().parseInt
    s = read()

  if p == 2:
    echo solve2(s)
    return
  if p == 5:
    echo solve5(s)
    return

  var
    freq = newSeq[int64](p)
    x = 0
    o = 1
    ans: int64 = 0
  freq[0] = 1
  for c in s.reversed:
    let t = (c.ord - '0'.ord) * o mod p
    x = (t + x) mod p
    ans += freq[x]
    freq[x] += 1
    o = o * 10 mod p
  echo ans

main()
