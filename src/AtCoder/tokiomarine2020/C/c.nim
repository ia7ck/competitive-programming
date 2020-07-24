import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc solve(n, k: int, aa: seq[int64]): seq[int64] =
  var a = aa
  for t in 0..<k:
    var b = newSeq[int64](n)
    for i in 0..<n:
      b[max(0, i - a[i])] += 1
      let r = i + a[i]
      if r + 1 < n:
        b[r + 1] -= 1
    for i in 1..<n:
      b[i] += b[i - 1]
    swap(a, b)
  return a

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
  let ans = if k >= 40:
    newSeqWith(n, n.int64)
  else:
    solve(n, k, a)
  echo ans.join(" ")

proc test() =
  const n = 2 * 100000
  var a = newSeq[int64](n)
  a[0] = 1
  for k in 1..(2 * 100000):
    echo k
    let res = solve(n, k, a)
    if res.allIt(it == n):
      echo k
      break
    if k == 40:
      echo res

test()
# main()
