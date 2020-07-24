import strutils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    s = read()
  var a = newSeq[int](n)
  for i in 0..<n:
    if s[i] == 'R':
      a[i] = 0
    elif s[i] == 'G':
      a[i] = 1
    else:
      a[i] = 2
  var ans: int64 = 0
  for j in 0..<n:
    var
      cnt = newSeq[int](n)
      tnc = newSeq[int](n)
    for i in 0..<j:
      if a[i] == (a[j] + 1) mod 3:
        cnt[j - i] += 1
      if a[i] == (a[j] + 2) mod 3:
        tnc[j - i] += 1
    var
      r = 0
      s = 0
    for k in (j + 1)..<n:
      if a[k] == (a[j] + 2) mod 3:
        r += 1
        ans -= cnt[k - j]
      if a[k] == (a[j] + 1) mod 3:
        s += 1
        ans -= tnc[k - j]
    ans += cnt.sum * r
    ans += tnc.sum * s
  echo ans
main()
