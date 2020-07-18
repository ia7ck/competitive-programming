import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, C = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var
    last = newSeqWith(C + 1, -1)
    ans = newSeqWith(C + 1, (n + 1) * n div 2)
  for i in 0..<n:
    if last[a[i]] == -1:
      ans[a[i]] -= (i + 1) * i div 2
    else:
      let m = i - last[a[i]] - 1
      # echo (a[i], i, last[a[i]])
      ans[a[i]] -= (m + 1) * m div 2
    last[a[i]] = i
    # echo ans
  for k in 1..C:
    let m = n - last[k] - 1
    ans[k] -= (m + 1) * m div 2
  echo ans[1..C].mapIt($it).join("\n")
main()

# 1 2 3 4 5 3 6 7 8 3 9
