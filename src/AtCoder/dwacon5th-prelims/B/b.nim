import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  var cul = newSeq[int64](n + 1)
  for i in 0..<n:
    cul[i + 1] = cul[i] + a[i]
  var b = newSeq[int64]()
  for i in 0..<n:
    for j in i..<n:
      b.add(cul[j + 1] - cul[i])

  # 上の桁から決める
  # '1' が K 個以上あるなら、それらを残す
  # K 個未満ならぜんぶ残す
  for i in countdown(50, 0):
    var nxt = b.filterIt(((it shr i) and 1) == 1)
    if nxt.len >= k:
      swap(b, nxt)
  var ans = b[0]
  for it in b:
    ans = ans and it
  echo ans
main()
