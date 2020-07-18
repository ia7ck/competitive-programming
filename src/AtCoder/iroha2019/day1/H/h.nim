import strutils, sequtils

proc dsum(n: int64): int =
  var ten: int64 = 1
  while n div ten > 0:
    result += (n div ten) mod 10
    ten *= 10

proc main() =
  #  3 => 12
  # 19 => 28
  # 99 => 189 ?
  let n = stdin.readLine.strip.parseInt

  var
    s = dsum(n)
    k: int64 = s mod 9
  for i in 0..<(s div 9):
    k = k * 10 + 9
  if k != n:
    echo k
    return
  var ten: int64 = 10 # ?
  while (k div ten) mod 10 == 9:
    ten *= 10
  k = k + ten - (ten div 10)
  doAssert(dsum(n) == dsum(k))
  echo k
main()
