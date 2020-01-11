import strutils, sequtils, algorithm

proc sub(n: int): seq[int] =
  var a = newSeq[int](n)
  for i in 0..<n:
    a[i] = i
  var b = newSeq[int](n)
  for i in 0..<n:
    b[i] = 1
  while a.nextPermutation:
    var seen = newSeq[bool](n)
    for it in a:
      seen[it] = true
      var j = it
      while j < n and seen[j]:
        b[j] += 1
        j += 1
  return b

proc mpow(a, x, m: int64): int64 =
  if x == 0: return 1
  if x == 1: return a
  if x mod 2 == 1: return a * mpow(a, x - 1, m) mod m
  return mpow(a * a mod m, x div 2, m)

proc main() =
  if false:
    for n in 1..10:
      echo n
      let b = sub(n)
      echo b
      echo ((1..<n).mapIt(b[it] - b[it - 1]))

  let n = stdin.readLine.strip.parseInt
  let x = stdin.readLine.strip.split.map(parseBiggestInt)

  const mo: int64 = 1000000000 + 7
  var y = newSeq[int64](n - 1)
  y[0] = 1
  for i in 1..(n - 1):
    y[0] = y[0] * i mod mo
  var d = y[0]
  for i in 1..<(n - 1):
    d = d * i mod mo * mpow(i + 1, mo - 2, mo) mod mo
    y[i] = (y[i - 1] + d) mod mo
  var ans: int64 = 0
  for i in 0..<(n - 1):
    let z = (x[i + 1] - x[i]) * y[i] mod mo
    ans = (ans + z) mod mo
  echo ans
main()
