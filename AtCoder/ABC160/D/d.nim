import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, x, y = read().parseInt

  var ans = newSeq[int](n)
  for i in 1..n:
    for j in (i + 1)..n:
      var d = j - i
      if j <= x or y <= i:
        discard
      else:
        d = min(d, abs(x - i) + abs(y - j) + 1)
      ans[d] += 1

  echo ans[1..(n - 1)].mapIt($it).join("\n")


main()
