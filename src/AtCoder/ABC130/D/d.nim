import strutils, sequtils

proc main() =
  var nn, k: int64
  (nn, k) = stdin.readLine.strip.split.map(parseBiggestInt)
  let n = nn.int
  let a = stdin.readLine.strip.split.map(parseBiggestInt)

  var
    i = 0
    j = 0
    s: int64 = 0
    ans: int64 = 0
  while i < n:
    while j < n and s < k:
      s += a[j]
      j += 1
    if s >= k:
      ans += n - j + 1
    s -= a[i]
    i += 1
  echo ans
main()
