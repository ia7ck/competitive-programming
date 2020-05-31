import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  # 36
  # => 2
  let n = read().parseBiggestInt

  if n == 1:
    echo 0
    return
  var
    factors = newSeq[(int64, int)]()
    nn = n
    i: int64 = 2
  while i * i <= n:
    if nn mod i == 0:
      var e = 0
      while nn mod i == 0:
        e += 1
        nn = nn div i
      factors.add((i, e))
    i += 1
  if nn > 1:
    factors.add((nn, 1))
  var ans = 0
  for (p, e) in factors:
    # 1 + 2 + 3 + ... + k = k * (k + 1) / 2 <= e
    var k = 0
    while k * (k + 1) div 2 <= e:
      k += 1
    ans += k - 1
  echo ans
main()
