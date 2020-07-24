import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseBiggestInt)

  var xsum: int64 = 0
  for it in a:
    xsum = xsum xor it
  var b = newSeq[int64]()
  for it in a:
    b.add(it and (not xsum)) #
  # echo xsum.toBin(8)
  # echo b.mapIt(it.toBin(8)).join("\n")
  var r: int = 0
  for j in countdown(59, 0):
    for i in r..<n:
      if ((b[i] shr j) and 1) == 1:
        for k in 0..<n:
          if i == k: continue
          if ((b[k] shr j) and 1) == 1:
            b[k] = b[k] xor b[i]
        # 2^j に 1 が立ってるのは b[i] だけ
        swap(b[r], b[i])
        r += 1
        break
  var xs2: int64 = 0
  for it in b:
    xs2 = xs2 xor it
  echo xsum + xs2 * 2

main()
