import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
  var
    i = 1
    j = n - 2
    s = a[0]
    t = a[n - 1]
    ans: int64 = 0
  while i <= j:
    let
      ll = s + 1 + s + a[i]
      rr = t + 1 + t + a[j]
    if ll < rr:
      ans += ll
      s += a[i] + a[i + 1] + 2
      i += 2
    else:
      ans += rr
      t += a[j] + a[j - 1] + 2
      j -= 2
  echo ans
main()
