import strutils, sequtils, algorithm, deques

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt
  var a = newSeqWith(n, read().parseBiggestInt)

  a.sort(cmp)
  a.reverse
  var
    q = initDeque[int64]()
    ans: int64 = 0
  q.addLast(a[0])
  for i in 1..<n:
    let x = q.popFirst
    ans += x
    q.addLast(a[i])
    q.addLast(a[i])
  echo ans
main()
