import strutils, sequtils, algorithm, math

proc main() =
  let
    s = stdin.readLine.strip
    n = s.len

  var a = newSeqWith(n + 1, -1)
  if s[0] == '<':
    a[0] = 0
  if s[^1] == '>':
    a[^1] = 0
  for i in 1..<n:
    if s[i] == '>' and s[i + 1] == '<':
      a[i + 1] = 0
  for i in 0..n:
    if a[i] == 0:
      var j = i - 1
      while j >= 0 and s[j] == '>':
        a[j] = max(a[j], a[j + 1] + 1)
        j -= 1
      j = i + 1
      while j <= n and s[j - 1] == '<':
        a[j] = max(a[j], a[j - 1] + 1)
        j += 1
  echo a.sum
main()
