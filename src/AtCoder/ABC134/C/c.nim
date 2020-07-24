import strutils, sequtils, math

proc main() =
  let n = stdin.readLine.strip.parseInt
  let a = (0..<n).mapIt(stdin.readLine.strip.parseInt)
  var
    f = newSeq[int](n)
    b = newSeq[int](n)
  for i in 0..<n:
    f[i] = a[i]
    if i > 0:
      f[i] = max(f[i], f[i - 1])
  for i in countdown(n - 1, 0):
    b[i] = a[i]
    if i + 1 < n:
      b[i] = max(b[i], b[i + 1])
  for i in 0..<n:
    if i == 0:
      echo b[1]
    elif i + 1 == n:
      echo f[n - 2]
    else:
      echo max(f[i - 1], b[i + 1])

main()
