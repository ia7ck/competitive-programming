import strutils, sequtils, math, algorithm

proc z_algo(s: string): seq[int] =
  let n = s.len
  var z = newSeq[int](n)
  z[0] = n
  var
    i = 1
    j = 0
  while i < n:
    while i + j < n and s[j] == s[i + j]: j += 1
    if j == 0:
      i += 1
      continue
    z[i] = j
    var k = 1
    while i + k < n and k + z[k] < j:
      z[i + k] = z[k]
      k += 1
    i += k
    j -= k
  return z

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip

  var ans = 0
  for i in 0..<n:
    let z = z_algo(s[i..^1])
    for j in (i + 1)..<n:
      ans = max(ans, min(j - i, z[j - i]))
  echo ans
main()
