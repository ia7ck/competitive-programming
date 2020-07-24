import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseInt)
  var
    freq = newSeq[int](100001)
    i = 0
    j = 0
    ans = 0
  freq[a[0]] += 1
  while j < n:
    # a[i], a[i + 1], ..., a[j]
    while j < n:
      ans = max(ans, j - i + 1)
      j += 1
      if j < n:
        freq[a[j]] += 1
        if freq[a[j]] >= 2:
          break
    while i < j:
      freq[a[i]] -= 1
      if freq[a[i]] == 1:
        i += 1
        break
      i += 1
  echo ans
main()
