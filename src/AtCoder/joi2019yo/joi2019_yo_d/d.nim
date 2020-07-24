import strutils, sequtils, algorithm, future

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  if a.allIt(it == 0):
    echo 0
    return
  var iota = newSeq[int](n)
  for i in 0..<n:
    iota[i] = i
  iota.sort((i, j) => (-cmp(a[i], a[j])))
  var
    i = 0
    cnt = 0
    ans = 0
    seen = newSeq[bool](n)
  while i < n:
    let y = a[iota[i]]
    while i < n and y == a[iota[i]]:
      let j = iota[i]
      cnt += 1
      seen[j] = true
      if j > 0 and seen[j - 1]:
        cnt -= 1
      if j + 1 < n and seen[j + 1]:
        cnt -= 1
      i += 1
    ans = max(ans, cnt)
  echo ans
main()
