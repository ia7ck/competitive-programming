import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  var
    right = newSeq[int](n + 1)
    left = newSeq[int](n + 1)
  right[n - 1] = 1
  for i in countdown(n - 2, 0):
    if a[i] != a[i + 1]:
      right[i] = right[i + 1] + 1
    else:
      right[i] = 1
  var
    i = 0
    ans = 1
  left[0] = 1
  for j in 1..n:
    if j == n or a[j - 1] == a[j]:
      var len = j - i
      if i > 0 and a[i - 1] == a[i]:
        len += left[i - 1]
      if j < n and a[j - 1] == a[j]:
        len += right[j]
      ans = max(ans, len)
      i = j
      left[j] = 1
    else:
      left[j] = left[j - 1] + 1
  echo ans
main()
