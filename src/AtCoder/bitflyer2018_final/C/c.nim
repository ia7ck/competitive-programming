import strutils, sequtils, math

proc main() =
  let s = stdin.readLine.strip

  var
    h = 100000
    left = newSeq[int](200002)
    ans: int64 = 0
  for c in s:
    if c == '(':
      left[h] += 1
      h += 1
    else:
      left[h] = 0
      h -= 1
      ans += left[h]
  echo ans
main()
