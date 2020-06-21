import strutils, sequtils, tables

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
    Q = read().parseInt
  var
    freq = initTable[int64, int64]()
    tot: int64 = 0
  for x in a:
    if freq.contains(x):
      freq[x] += 1
    else:
      freq[x] = 1
    tot += x
  for q in 0..<Q:
    let b, c = read().parseBiggestInt
    let
      fb = freq.getOrDefault(b)
      fc = freq.getOrDefault(c)
    freq[b] = 0
    if freq.contains(c):
      freq[c] += fb
    else:
      freq[c] = fb
    tot -= b * fb
    tot += c * fb
    echo tot
main()
