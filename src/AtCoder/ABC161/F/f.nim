import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseBiggestInt

  var
    d: int64 = 2
    ans: int64 = 0
  while d * d <= n - 1:
    if (n - 1) mod d == 0:
      if d * d == n - 1:
        ans += 1
      else:
        ans += 2
    d += 1
  d = 2
  proc check(k: int64): bool =
    var m = n
    while m mod k == 0:
      m = m div k
    if m == 1:
      return true
    elif m >= k and (m - 1) mod k == 0:
      return true
    return false
  while d * d <= n:
    if n mod d == 0:
      if d * d == n:
        if check(d):
          ans += 1
      else:
        if check(d):
          ans += 1
        if check(n div d):
          ans += 1
    d += 1
  if n - 1 >= 2:
    ans += 1
  echo ans + 1
main()
