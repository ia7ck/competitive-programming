import strutils, sequtils
proc main()=
  let n = stdin.readLine.parseInt * 2
  var ans = 0
  for bits in 0..<(1 shl n):
    var 
      cur = 0
      ok = true
    for i in 0..<n:
      if (bits and (1 shl i)) > 0:
        cur += 1
      else:
        cur -= 1
      if cur < 0:
        ok = false
        break
    if ok and (cur == 0):
      ans += 1
  echo ans
main()
