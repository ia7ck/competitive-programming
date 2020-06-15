import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    s = read()
    n = s.len

  var ans = 10
  for bits in 1..<(1 shl n):
    var
      tt = newSeq[string]()
      r = 0
    for i in 0..<n:
      if ((bits shr i) and 1) == 1:
        var t = 'x'.repeat(i)
        while t.len < 40:
          t &= s
        tt.add(t)
        r = i
    var a = newSeq[int](40)
    for t in tt:
      for j in 0..<40:
        if t[j] == 'o':
          a[j] += 1
    # echo tt
    # echo a
    if a[r..^1].allIt(it >= 1):
      ans = min(ans, tt.len)
  echo ans
main()
