import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc isPrime(k: int): bool =
  var d = 2
  while d * d <= k:
    if k mod d == 0:
      return false
    d += 1
  return true

proc judge(a: seq[int]): bool =
  var ord = newSeq[int](a.len)
  for i in 0..<5:
    ord[i] = 1
  ord.sort(cmp)
  while true:
    var s = 0
    for i in 0..<ord.len:
      if ord[i] == 1:
        s += a[i]
    if isPrime(s):
      return false
    if not ord.nextPermutation:
      break
  return true

proc main() =
  let n = read().parseInt

  var
    a = newSeq[int]()
    k = 3
  while a.len < n:
    if isPrime(k):
      a.add(k)
    k += 10

  doAssert(judge(a))
  echo a.mapIt($it).join(" ")

main()
