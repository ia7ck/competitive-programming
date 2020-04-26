import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)
    mx = a.max
    mn = a.min
    mxIdx = a.find(mx)
    mnIdx = a.find(mn)
  var
    b = newSeq[int64](n)
    ans = newSeq[(int, int)]()
  if mx.abs >= mn.abs:
    for i in 0..<n:
      b[i] = a[i] + mx
      ans.add((mxIdx, i))
    for i in 1..<n:
      b[i] = b[i] + b[i - 1]
      ans.add((i - 1, i))
  else:
    for i in 0..<n:
      b[i] = a[i] + mn
      ans.add((mnIdx, i))
    for i in countdown(n - 2, 0):
      b[i] = b[i] + b[i + 1]
      ans.add((i + 1, i))
  echo ans.len
  for it in ans:
    let (x, y) = it
    echo x + 1, " ", y + 1
main()
