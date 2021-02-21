import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let Q = read().parseInt
  let L = read().parseBiggestInt
  type P = tuple[val, cnt: int64]
  var
    size: int64 = 0
    stk = newSeq[P]()
  for qq in 0..<Q:
    let t = read()
    if t == "Push":
      let n, m = read().parseBiggestInt
      stk.add((m, n))
      size += n
      if size > L:
        echo "FULL"
        return
    elif t == "Pop":
      var n = read().parseBiggestInt
      if size < n:
        echo "EMPTY"
        return
      size -= n
      while n > 0 and stk.len > 0:
        if stk[^1].cnt <= n:
          n -= stk[^1].cnt
          discard stk.pop
        else:
          stk[^1].cnt -= n
          n = 0
    elif t == "Top":
      if stk.len == 0:
        echo "EMPTY"
        return
      echo stk[^1].val
    else:
      echo size
  echo "SAFE"
main()
